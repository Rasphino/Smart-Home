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
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::HashMap;

use config;

use crate::aliyun::{Aliyun, AliyunBuilder};
use crate::dht11::Dht11Data;

fn main() {
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

    // thread::spawn(move || {
    //     loop {
    //         hc_tx.send(hcsr04::collect_data()).unwrap();
    //         thread::sleep(Duration::from_secs(10));
    //     }
    // });

    let data = Arc::new(Mutex::new(Dht11Data::new()));
    {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(5));
                let d = *data.lock().unwrap();
                println!("Send temperature: {} and humidity: {} to server", d.temperature, d.humidity);
                aliyun.push(&d);
            }
        });
    }

    for received in rx {
        match received {
            None => eprintln!("Failed reading data"),
            Some(d) => {
                *data.lock().unwrap() = d;
                // println!("Got temperature: {} and humidity: {}", temperature, humidity)
            }
            // Some(Data::HCSR04(distance)) => println!("Got distance: {}", distance),
        }
    }
}
