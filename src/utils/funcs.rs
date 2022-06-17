
use std::{fs::File, io::{Write, Stdout}, collections::HashMap, time::Instant};
use crossterm::{terminal::{Clear, ClearType}, ExecutableCommand, cursor::MoveTo};
use serde_derive::Deserialize;
use reqwest::blocking::Client;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    pub contacts: Contacts,
    pub timings: Timings,
    pub thresholds: Thresholds,
}

impl Config {
    pub fn new(webhook: String, webhook_pause:u64,interval:u64,used_mem:u8,load_1m: u32) -> Config {
        Config { 
            contacts: Contacts { webhook}, 
            timings: Timings { webhook_pause, interval},
            thresholds: Thresholds {used_mem, load_1m},
        }
    }
}


#[derive(Deserialize)]
#[derive(Debug)]
pub struct Contacts {
    pub webhook: String, 
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Timings {
    pub webhook_pause: u64,
    pub interval: u64,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Thresholds {
    pub used_mem: u8,
    pub load_1m: u32,
}

pub fn send_post(desc: String,url:&str) {
    let mut map = HashMap::new();
    map.insert("content", desc);

    let client = Client::new();
    let resp = client.post(url)
        .json(&map)
        .send();

    if resp.is_err() {
        eprintln!("Error sending POST request");
    }
    
}

pub fn create_default_toml()
{
    let mut file = File::create("snmpfetch_config.toml").unwrap();
    file.write_all(b"[contacts]
webhook = \"\"
    
[timings]
# Seconds to wait
webhook_pause = 3600
interval = 1

[thresholds]
# Percentage of used memory
used_mem = 80
# Load 1 minute value
load_1m = 500").unwrap();
}

pub fn check_time_passed(origin_secs: Instant, threshhold: u64) -> bool {
    let now = Instant::now();
    let diff = now.duration_since(origin_secs);
    let diff_secs = diff.as_secs();
    if diff_secs >= threshhold {
        return true;
    }
    else {
        return false;
    } 
}

pub fn cls(stdout:&mut Stdout)
{
    match stdout.execute(Clear(ClearType::All))
    {
        Ok(_) => {
            match stdout.execute(MoveTo(0,0))
            {
                _ => (),
            }
        },
        Err(_) => eprintln!("Error clearing screen"),
    }
}


#[allow(unused_assignments)]
pub fn sec_to_date(mut secs: u64) -> String {
    let mut years = 0;
    let mut days = 0;
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;
    let mut result = String::new();

    if secs >= 31536000 {
        years = secs / 31536000;
        secs = secs % 31536000;
        result.push_str(&format!("{}y ", years));
    }
    if secs >= 86400 {
        days = secs / 86400;
        secs = secs % 86400;
        result.push_str(&format!("{}d ", days));
    }
    if secs >= 3600 {
        hours = secs / 3600;
        secs = secs % 3600;
        result.push_str(&format!("{}h ", hours));
    }
    if secs >= 60 {
        minutes = secs / 60;
        secs = secs % 60;
        result.push_str(&format!("{}m ", minutes));
    }
    seconds = secs;
    result.push_str(&format!("{}s", seconds));

    result
}