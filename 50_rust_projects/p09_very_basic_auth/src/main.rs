use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    println!("Very Basic Authentication");
    let client = Client::new();

    let user = "test_user".to_string();
    let passwd: Option<String> = None;

    let response = client
        .get("http://httpbin.org/get")
        .basic_auth(user, passwd)
        .send();

    println!("{:?}", response);
    Ok(())
}
