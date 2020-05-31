use std::collections::HashMap;

use reqwest::blocking::Client;
use crypto::{md5, hmac};
use crypto::mac::Mac;
use hex;

#[derive(Serialize, Deserialize, Debug)]
struct LoginRespond {
    code: i32,
    info: Info,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    token: String
}

pub fn login(configurations: &HashMap<String, String>) -> Result<String, reqwest::Error> {
    let device_secret = configurations.get("device_secret").unwrap().clone();
    let client_id = configurations.get("client_id").unwrap().clone();
    let device_name = configurations.get("device_name").unwrap().clone();
    let product_key = configurations.get("product_key").unwrap().clone();

    // generate HmacMd5 and calculate sign for login
    let mut mac = hmac::Hmac::<md5::Md5>::new(md5::Md5::new(), device_secret.as_bytes());
    mac.input(format!("clientId{}deviceName{}productKey{}",
                      client_id, device_name, product_key).as_bytes());
    let result = mac.result();
    let sign = hex::encode(result.code());

    let body = json!({
        "productKey": product_key,
        "deviceName": device_name,
        "clientId": client_id,
        "sign": sign
    });

    let client = Client::new();
    let res: LoginRespond = client.post("https://iot-as-http.cn-shanghai.aliyuncs.com/auth")
        .json(&body)
        .send()?
        .json()?;

    println!("Respond: {:?}", res);

    if res.code == 0 {
        Ok(res.info.token)
    } else {
        panic!("Server response code: {}, and message: {}", res.code, res.message)
    }
}