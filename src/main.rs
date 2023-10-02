use reqwest::blocking::Client;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    let client = Client::new;


    let url: String;

    if args.len() != 2 {
        url = "ipinfo.io/json".to_string();
    } else {
        let ip = &args[1];
        url = format!("ipinfo.io/{ip}/json");
    }

    println!("{}", url);
}
