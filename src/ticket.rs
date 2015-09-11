use hyper;
use url::percent_encoding::{utf8_percent_encode_to, FORM_URLENCODED_ENCODE_SET};
use serde_json;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Ticket {
    characters: Vec<String>,
    default_character: String,
    ticket: String,
    account_id: i64,
    friends: serde_json::Value,
    bookmarks: serde_json::Value,
    error: String,
}

impl Ticket {
    pub fn request(username: &str, password: &str) -> Ticket {
        let mut client = hyper::Client::new();
        let mut body = "account=".to_string();
        utf8_percent_encode_to(username, FORM_URLENCODED_ENCODE_SET, &mut body);
        body.push_str("&password=");
        utf8_percent_encode_to(password, FORM_URLENCODED_ENCODE_SET, &mut body);
        let mime = "application/x-www-form-urlencoded".parse().unwrap();
        let mut response = client.post("http://www.f-list.net/json/getApiTicket.php")
            .body(&body)
            .header(hyper::header::ContentType(mime))
            .send().unwrap();
        let mut response_string = String::new();
        response.read_to_string(&mut response_string).unwrap();
        println!("{}", response_string);
        let ticket: Ticket = serde_json::from_str(&response_string).unwrap(); // TODO: Handle the possibility of an response with an error.
        ticket
    }

    pub fn characters(&self) -> Vec<String> {
        self.characters.clone()
    }
}
