use reqwest::blocking::Client;
use serde_json::Value;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();


    let url: String;

    if args.len() != 2 {
        url = "https://ipinfo.io/json".to_string();
    } else {
        let ip = &args[1];
        url = format!("https://ipinfo.io/{ip}/json");
    }


    let client = Client::new();

    let resp: Value = client
        .get(url)
        .send()
        .expect("Ipinfo did a little trolling")
        .json()
        .unwrap();


    if &resp["bogon"] == true {
        println!("Unable to find IP!");
        return
    }

    println!("IP: {}",&resp["ip"]);
    println!("City: {}",&resp["city"]);
    println!("Region: {}",&resp["region"]);
    println!("Country: {}",&resp["country"]);
    println!("Location: {}",&resp["loc"]);
    println!("Orginization: {}",&resp["org"]);
    println!("Postal Code: {}",&resp["postal"]);
    println!("Timezone: {}",&resp["timezone"]);



}
