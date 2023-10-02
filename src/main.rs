use reqwest::blocking::Client;
use serde_json::Value;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();


    let url: String;

    if args.len() != 2 {
        url = "ipinfo.io/json".to_string();
    } else {
        let ip = &args[1];
        url = format!("ipinfo.io/{ip}/json");
    }

    println!("{}", url);
    let client = Client::new();


    let res: Value = client
    .get(url)
    .send()
    .expect("Unable to send request")
    .json()
    .unwrap();



}
