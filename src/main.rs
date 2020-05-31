mod data;
mod DHT11;
mod HCSR04;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use data::Data;

fn main() {
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
