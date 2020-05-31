use rand::Rng;

use crate::data::Data;

pub fn collect_data() -> Option<Data> {
    let fake_data = rand::thread_rng().gen_range(20, 45);
    if fake_data > 40 {
        None
    } else {
        Some(Data::DHT11(fake_data, 56))
    }
}