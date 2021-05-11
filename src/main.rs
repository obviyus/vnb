use chrono::Local;
use log::warn;
use reqwest::StatusCode;
use std::{collections::HashMap, thread::sleep};
use std::{env, time};
use teloxide::prelude::*;
use teloxide::utils::markdown::*;

mod response;
use crate::response::*;
#[derive(Default, Debug, Clone)]
struct Slot {
    center_name: String,
    pincode: String,
    available_capacity: String,
    date: String,
    vaccine_name: Option<String>,
}

async fn scan_district(district_id: u16) -> Result<Option<Vec<Slot>>, Box<dyn std::error::Error>> {
    let today_date = Local::now().format("%d-%m-%Y");
    let url =
        format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/public/calendarByDistrict?district_id={}&date={}",
                district_id, today_date);

    let resp: Root = match reqwest::get(url).await {
        Ok(r) => {
            if r.status() == StatusCode::FORBIDDEN {
                warn!("API limit reached. Sleeping for 10s");

                sleep(time::Duration::from_secs(10));
                return Ok(None);
            } else {
                match r.json::<Root>().await {
                    Ok(val) => val,
                    _ => return Ok(None),
                }
            }
        }
        _ => return Ok(None),
    };

    let mut available_centers: Vec<Slot> = Vec::new();
    for center in resp.centers.iter() {
        for session in center.sessions.iter() {
            if session.min_age_limit < 45 && session.available_capacity > 1 {
                let slot: Slot = Slot {
                    center_name: center.name.clone(),
                    pincode: format!("{}", center.pincode),
                    available_capacity: format!("{}", session.available_capacity),
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
        0 => Ok(None),
        _ => Ok(Some(available_centers)),
    }
}

async fn send_message(
    channel_id: String,
    slots: Vec<Slot>,
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
        .send_message(channel_id, format!("{}", constructed_message))
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
    teloxide::enable_logging_with_filter!(log::LevelFilter::Debug);
    let bot = Bot::from_env();

    // Send a message to the owner when bot starts
    match env::var("OWNER_ID") {
        Ok(owner_id) => start_message(owner_id, &bot).await.unwrap(),
        _ => warn!("No OWNER_ID set"),
    }

    // List of monitored districts 
    // TODO: Import from a file
    let monitored_districts: HashMap<u16, &str> = [
        (8, "Visakhapatnam"),
        (49, "Kamrup Metropolitan"),
        (64, "Sonitpur"),
        (74, "Araria"),
        (86, "Muzaffarpur"),
        (97, "Patna"),
        (108, "Chandigarh"),
        (109, "Raipur"),
        (142, "West Delhi"),
        (145, "East Delhi"),
        (146, "North Delhi"),
        (150, "South West Delhi"),
        (151, "North Goa"),
        (152, "South Goa"),
        (154, "Ahmedabad"),
        (187, "Panchkula"),
        (188, "Gurgaon"),
        (192, "Ambala"),
        (195, "Panipat"),
        (199, "Faridabad"),
        (202, "Rewari"),
        (212, "Sirmaur"),
        (265, "Bangalore Urban"),
        (266, "Mysore"),
        (269, "Dakshina Kannada"),
        (281, "Uttar Kannada"),
        (286, "Udupi"),
        (294, "BBMP"),
        (296, "Thiruvananthapuram"),
        (297, "Kannur"),
        (300, "Pathanamthitta"),
        (301, "Alappuzha"),
        (302, "Malappuram"),
        (303, "Thrissur"),
        (304, "Kottayam"),
        (307, "Ernakulam"),
        (308, "Palakkad"),
        (312, "Bhopal"),
        (313, "Gwalior"),
        (316, "Rewa"),
        (348, "Guna"),
        (362, "Betul"),
        (363, "Pune"),
        (365, "Nagpur"),
        (376, "Satara"),
        (390, "Jalgaon"),
        (391, "Ahmednagar"),
        (392, "Thane"),
        (393, "Raigad"),
        (395, "Mumbai"),
        (397, "Aurangabad"),
        (446, "Khurda"),
        (453, "Sundargarh"),
        (457, "Cuttack"),
        (494, "Patiala"),
        (496, "SAS Nagar"),
        (501, "Bikaner"),
        (502, "Jodhpur"),
        (505, "Jaipur I"),
        (507, "Ajmer"),
        (512, "Alwar"),
        (513, "Sikar"),
        (521, "Chittorgarh"),
        (523, "Bhilwara"),
        (530, "Churu"),
        (571, "Chennai"),
        (581, "Hyderabad"),
        (624, "Prayagraj"),
        (650, "Gautam Buddha Nagar"),
        (651, "Ghaziabad"),
        (664, "Kanpur Nagar"),
        (670, "Lucknow"),
        (676, "Meerut"),
        (679, "Muzaffarnagar"),
        (689, "Shamli"),
        (696, "Varanasi"),
        (697, "Dehradun"),
        (709, "Nainital"),
        (721, "Howrah"),
        (725, "Kolkata"),
        (773, "Jamnagar Corporation"),
        (775, "Rajkot Corporation"),
        (777, "Vadodara Corporation"),
    ]
    .iter()
    .cloned()
    .collect();

    loop {
        for (district_id, district_name) in monitored_districts.iter() {
            match scan_district(*district_id).await {
                Ok(val) => match val {
                    Some(centers) => match env::var("CHANNEL_ID") {
                        Ok(channel_id) => send_message(channel_id, centers, district_name, &bot)
                            .await
                            .unwrap(),
                        _ => panic!("No CHANNEL_ID set"),
                    },
                    _ => (),
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
