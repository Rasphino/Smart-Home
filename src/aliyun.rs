use std::collections::HashMap;

use reqwest::blocking::Client;
use crypto::{md5, hmac};
use crypto::mac::Mac;
use hex;
use chrono::Utc;

use crate::dht11::Dht11Data;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    code: i32,
    info: Info,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    token: Option<String>,
}

pub struct Aliyun {
    device_secret: String,
    client_id: String,
    device_name: String,
    product_key: String,
    token: String,
}

pub struct AliyunBuilder<'a> {
    device_secret: &'a str,
    client_id: &'a str,
    device_name: &'a str,
    product_key: &'a str,
}

impl<'a> AliyunBuilder<'a> {
    pub fn new(configurations: &'a HashMap<String, String>) -> AliyunBuilder<'a> {
        AliyunBuilder {
            device_secret: configurations.get("device_secret").unwrap(),
            client_id: configurations.get("client_id").unwrap(),
            device_name: configurations.get("device_name").unwrap(),
            product_key: configurations.get("product_key").unwrap(),
        }
    }

    pub fn login(&mut self) -> Aliyun {
        // generate HmacMd5 and calculate sign for login
        let mut mac = hmac::Hmac::<md5::Md5>::new(md5::Md5::new(), self.device_secret.as_bytes());
        mac.input(format!("clientId{}deviceName{}productKey{}",
                          self.client_id, self.device_name, self.product_key).as_bytes());
        let result = mac.result();
        let sign = hex::encode(result.code());

        let body = json!({
            "productKey": self.product_key,
            "deviceName": self.device_name,
            "clientId": self.client_id,
            "sign": sign
        });

        let res: Response = Client::new().post("https://iot-as-http.cn-shanghai.aliyuncs.com/auth")
            .json(&body)
            .send().unwrap()
            .json().unwrap();

        println!("Respond: {:?}", res);

        if res.code == 0 {
            Aliyun {
                device_secret: self.device_secret.to_string(),
                client_id: self.client_id.to_string(),
                device_name: self.device_name.to_string(),
                product_key: self.product_key.to_string(),
                token: res.info.token.unwrap(),
            }
        } else {
            panic!("Server response code: {}, and message: {}", res.code, res.message)
        }
    }
}

impl Aliyun {
    pub fn push(&self, data: &Dht11Data) {
        let body = json!({
            "id": Utc::now().timestamp(),
            "params": {
                "temperature": data.temperature,
                "humidity": data.humidity
            },
            "method": "thing.event.property.post"
        }).to_string();
        println!("Body: {}", body);

        let post_url = format!("https://iot-as-http.cn-shanghai.aliyuncs.com/topic/sys/{}/{}/thing/event/property/post", self.product_key, self.device_name);
        let res: Response = Client::new().post(&post_url)
            .body(body)
            .header("password", &self.token)
            .header(reqwest::header::CONTENT_TYPE, "application/octet-stream")
            .send().unwrap()
            .json().unwrap();
        println!("Res: {:?}", res);
    }
}