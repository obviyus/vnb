use std::{collections::HashMap, env};

use backoff::ExponentialBackoff;
use chrono::{Datelike, Duration, Local};
use log::{info, warn};
use pretty_env_logger::formatted_timed_builder;
use reqwest::Client;
use teloxide::prelude::*;
use teloxide::utils::markdown::*;

use crate::response::*;

mod response;

// Struct to hold every valid slot
#[derive(Default, Debug, Clone, PartialEq)]
struct Slot {
    center_name: String,
    pincode: String,
    available_capacity: String,
    date: String,
    vaccine_name: Option<String>,
}

// Fetch CoWin response asynchronously with exponential backoff
async fn fetch_url(url: &str, client: &Client) -> Result<Root, reqwest::Error> {
    backoff::future::retry(ExponentialBackoff::default(), || async {
        info!("Fetching {}", url);
        Ok(client.get(url).send().await?.json::<Root>().await?)
    })
    .await
}

// Query CoWin for a given district_id for the next 14 days
#[allow(clippy::match_single_binding)]
async fn scan_district(district_id: u16, client: &Client) -> Option<Vec<Slot>> {
    let mut available_centers: Vec<Slot> = Vec::new();
    let mut date = Local::now();

    loop {
        let url =
            format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/public/calendarByDistrict?district_id={}&date={}",
                    district_id, date.format("%d-%m-%Y"));

        // Only proceed if we get a valid <Root> struct
        let resp: Root = match fetch_url(&url, client).await {
            Ok(val) => val,
            _ => {
                info!("No response");
                return None;
            }
        };

        for center in resp.centers.iter() {
            for session in center.sessions.iter() {
                if session.min_age_limit < 45 && session.available_capacity > 1.0 {
                    info!("found valid slot: {}", center.name);

                    // Store a valid vaccination slot in <Slot>
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

                    // Break after finding a single valid slot
                    available_centers.push(slot);
                    break;
                }
            }
        }

        // If no centers are found for this week, change query date to the next monday
        if available_centers.is_empty() {
            if (date - Local::now()).num_days() > 7 {
                break;
            } else {
                date = date
                    + (Duration::days(7)
                        - Duration::days(date.weekday().num_days_from_monday() as i64));
            }
        } else {
            break;
        }
    }

    // Empty vector implies no slots were found
    match available_centers.len() {
        0 => None,
        _ => Some(available_centers),
    }
}

async fn send_message(
    channel_id: &str,
    slots: &[Slot],
    district_name: &str,
    bot: &teloxide::Bot,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut constructed_message = format!("*{}*", district_name);

    for slot in slots.iter() {
        constructed_message.push_str(
            &format!(
                "\n\n{} | {} | {} slots | {}",
                code_inline(&slot.center_name),
                bold(&slot.pincode.to_string()),
                &slot.available_capacity,
                &slot.date
            )
            // Escape all MarkdownV2 reserved entities
            .replace("|", r"\|")
            .replace("-", r"\-")
            .replace("(", r"\(")
            .replace(")", r"\)"),
        );

        // Not all centres provide name of vaccine
        match &slot.vaccine_name {
            Some(vaccine) => {
                constructed_message
                    .push_str(&format!(" | {}", bold(&vaccine.to_string())).replace("|", r"\|"));
            }
            None => (),
        }
    }
    println!("{}", constructed_message);

    // FIXME: Set a default ParseMode
    bot.parse_mode("MarkdownV2".parse().unwrap())
        .send_message(channel_id.to_string(), constructed_message.to_string())
        .send()
        .await
        .expect("Failed to send message");

    Ok(())
}

// Message sent to owner at bot startup
async fn start_message(
    owner_id: String,
    bot: &teloxide::Bot,
) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now().to_rfc2822();
    bot.parse_mode("MarkdownV2".parse().unwrap())
        .send_message(owner_id, format!("Vaccine scanning started at `{}`", now))
        .send()
        .await
        .expect("Failed to send message");

    Ok(())
}

// Main logic of program
async fn run() {
    formatted_timed_builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    // Panics if TELOXIDE_TOKEN is not set
    let bot = Bot::from_env();

    // Send a message to the owner when bot starts
    match env::var("OWNER_ID") {
        Ok(owner_id) => start_message(owner_id, &bot).await.unwrap(),
        _ => warn!("No OWNER_ID set"),
    }

    // Required ID for channel to which Bot sends message to AND is an admin
    // Find yours using @userinfobot
    let channel_id: String;
    match env::var("CHANNEL_ID") {
        Ok(c) => channel_id = c,
        _ => panic!("No CHANNEL_ID set"),
    }

    // Since we make a lot of GET requests, it's recommended to create a client
    // with a default user_agent
    let client = reqwest::Client::builder()
        .user_agent(response::USER_AGENT)
        .build()
        .unwrap();

    // Dictionary of the last sent message for a district
    // Avoids repeating the same message to the channel
    let mut seen: HashMap<&u16, Vec<Slot>> = HashMap::new();

    loop {
        for (district_id, district_name) in response::MONITORED_DISTRICTS.iter() {
            info!("scanning {}", district_name);

            if let Some(centers) = scan_district(*district_id, &client).await {
                match seen.get(&district_id) {
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
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    run().await;
}
