extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod data;
mod dht11;
mod hcsr04;
mod aliyun;

use data::Data;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::collections::HashMap;
use chrono::Utc;

use config;

use aliyun::Aliyun;
use crate::aliyun::AliyunBuilder;

fn main() {
    let timestamp = Utc::now().timestamp();
    println!("Time: {}", timestamp);

    // read configuration from file and environment variable
    let mut configurations = config::Config::default();
    configurations
        .merge(config::File::with_name("Config")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    let configurations = configurations.try_into::<HashMap<String, String>>().unwrap();

    let aliyun = AliyunBuilder::new(&configurations).login();

    let (dht_tx, rx) = mpsc::channel();
    let hc_tx = mpsc::Sender::clone(&dht_tx);

    thread::spawn(move || {
        loop {
            dht_tx.send(dht11::collect_data()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        loop {
            hc_tx.send(hcsr04::collect_data()).unwrap();
            thread::sleep(Duration::from_secs(10));
        }
    });

    for received in rx {
        match received {
            None => eprintln!("Failed reading data"),
            Some(Data::DHT11(temperature, humidity)) => println!("Got temperature: {} and humidity: {}", temperature, humidity),
            Some(Data::HCSR04(distance)) => println!("Got distance: {}", distance),
        }
    }
}
