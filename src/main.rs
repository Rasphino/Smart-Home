mod data;
mod DHT11;
mod HCSR04;

use data::Data;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::collections::HashMap;

use config;
use crypto::{md5, hmac};
use crypto::mac::Mac;
use hex;

fn main() {
    // read configuration from file and environment variable
    let mut configurations = config::Config::default();
    configurations
        .merge(config::File::with_name("Config")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    let configurations = configurations.try_into::<HashMap<String, String>>().unwrap();

    // generate HmacMd5 and calculate sign for login
    let mut mac = hmac::Hmac::<md5::Md5>::new(md5::Md5::new(), configurations.get("device_secret").unwrap().as_bytes());
    mac.input(format!("clientId{}deviceName{}productKey{}",
                      configurations.get("client_id").unwrap(),
                      configurations.get("device_name").unwrap(),
                      configurations.get("product_key").unwrap()).as_bytes());
    let sign = mac.result().code();

    let (dht_tx, rx) = mpsc::channel();
    let hc_tx = mpsc::Sender::clone(&dht_tx);

    thread::spawn(move || {
        loop {
            dht_tx.send(DHT11::collect_data()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        loop {
            hc_tx.send(HCSR04::collect_data()).unwrap();
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
