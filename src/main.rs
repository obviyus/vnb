use std::{collections::HashMap, env};

use backoff::ExponentialBackoff;
use chrono::Local;
use log::{info, warn};
use pretty_env_logger::formatted_timed_builder;
use reqwest::Client;
use teloxide::prelude::*;
use teloxide::utils::markdown::*;

use crate::response::*;

mod response;

#[derive(Default, Debug, Clone, PartialEq)]
struct Slot {
    center_name: String,
    pincode: String,
    available_capacity: String,
    date: String,
    vaccine_name: Option<String>,
}

async fn fetch_url(url: &str, client: &Client) -> Result<Root, reqwest::Error> {
    backoff::future::retry(ExponentialBackoff::default(), || async {
        info!("Fetching {}", url);
        Ok(client.get(url).send().await?.json::<Root>().await?)
    })
    .await
}

async fn scan_district(district_id: u16, client: &Client) -> Option<Vec<Slot>> {
    let today_date = Local::now().format("%d-%m-%Y");
    let url =
        format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/public/calendarByDistrict?district_id={}&date={}",
                district_id, today_date);

    let resp: Root = match fetch_url(&url, client).await {
        Ok(val) => val,
        _ => {
            info!("No response, quitting.");
            return None;
        }
    };

    let mut available_centers: Vec<Slot> = Vec::new();
    for center in resp.centers.iter() {
        for session in center.sessions.iter() {
            if session.min_age_limit < 45 && session.available_capacity > 1.0 {
                info!("found valid slot: {}", center.name);

                let slot: Slot = Slot {
                    center_name: center.name.clone(),
                    pincode: format!("{}", center.pincode),
                    available_capacity: format!("{}", session.available_capacity.round()),
                    date: session.date.clone(),
                    vaccine_name: match session.vaccine.clone() {
                        name => {
                            if name.is_empty() {
                                None
                            } else {
                                Some(name)
                            }
                        }
                    },
                };
                available_centers.push(slot);

                break;
            }
        }
    }

    match available_centers.len() {
        0 => None,
        _ => Some(available_centers),
    }
}

async fn send_message(
    channel_id: &String,
    slots: &Vec<Slot>,
    district_name: &str,
    bot: &teloxide::Bot,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut constructed_message = format!("*{}*", district_name);

    for slot in slots.iter() {
        constructed_message.push_str(
            &format!(
                "\n\n{} | {} | {} slots | {}",
                code_inline(&slot.center_name),
                bold(&format!("{}", &slot.pincode)),
                &slot.available_capacity,
                &slot.date
            )
            .replace("|", r"\|")
            .replace("-", r"\-")
            .replace("(", r"\(")
            .replace(")", r"\)"),
        );

        match &slot.vaccine_name {
            Some(vaccine) => {
                constructed_message
                    .push_str(&format!(" | {}", bold(&format!("{}", vaccine))).replace("|", r"\|"));
            }
            None => (),
        }
    }

    bot.parse_mode("MarkdownV2".parse().unwrap())
        .send_message(channel_id.clone(), format!("{}", constructed_message))
        .send()
        .await
        .expect("Sending message.");

    Ok(())
}

async fn start_message(
    owner_id: String,
    bot: &teloxide::Bot,
) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now().to_rfc2822();
    bot.parse_mode("MarkdownV2".parse().unwrap())
        .send_message(owner_id, format!("Vaccine scanning started at `{}`", now))
        .send()
        .await
        .expect("Sending message.");

    Ok(())
}

async fn run() {
    formatted_timed_builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    let bot = Bot::from_env();

    // Send a message to the owner when bot starts
    match env::var("OWNER_ID") {
        Ok(owner_id) => start_message(owner_id, &bot).await.unwrap(),
        _ => warn!("No OWNER_ID set"),
    }

    let channel_id: String;
    match env::var("CHANNEL_ID") {
        Ok(c) => channel_id = c,
        _ => panic!("No CHANNEL_ID set"),
    }

    let client = reqwest::Client::builder()
        .user_agent(response::USER_AGENT)
        .build()
        .unwrap();

    let mut seen: HashMap<&u16, Vec<Slot>> = HashMap::new();

    loop {
        for (district_id, district_name) in response::MONITORED_DISTRICTS.iter() {
            info!("scanning {}", district_name);
            match scan_district(*district_id, &client).await {
                Some(centers) => match seen.get(&district_id) {
                    Some(value) => {
                        if *value != centers {
                            send_message(&channel_id, &centers, district_name, &bot)
                                .await
                                .unwrap();
                            seen.insert(district_id, centers.clone());
                        }
                    }
                    None => {
                        seen.insert(district_id, centers.clone());
                        send_message(&channel_id, &centers, district_name, &bot)
                            .await
                            .unwrap()
                    }
                },
                _ => (),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    run().await;
}
