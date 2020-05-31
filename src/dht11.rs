use rand::Rng;

use crate::data::Data;

#[derive(Clone, Copy)]
pub struct Dht11Data {
    pub temperature: i32,
    pub humidity: i32
}

impl Dht11Data {
    pub fn new() -> Self {
        Dht11Data { temperature: 0, humidity: 0 }
    }
}

pub fn collect_data() -> Option<Dht11Data> {
    let fake_data = rand::thread_rng().gen_range(20, 45);
    if fake_data > 40 {
        None
    } else {
        Some(Dht11Data{
            temperature: fake_data,
            humidity: 56
        })
    }
}