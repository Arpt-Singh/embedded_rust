use reqwest::blocking::Client;


fn main() {
    let http_client = Client::new();
    let http_result = http_client.get("https://www.youtube.com").send();

    if http_result.is_ok() {
        println!("Body: {:#?}",http_result.ok().unwrap().text());
    }
    else if http_result.is_err() {
        println!("Error occured:{:#?}",http_result.err());
    }
}
