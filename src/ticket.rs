use serde_json;
use reqwest;
use std::io::Read;

#[derive(Debug)]
pub enum Error {
    StdIo(::std::io::Error),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Fchat(String),
    Other,
}

impl From<::std::io::Error> for Error {
    fn from(val: ::std::io::Error) -> Self {
        Error::StdIo(val)
    }
}

impl From<reqwest::Error> for Error {
    fn from(val: reqwest::Error) -> Self {
        Error::Reqwest(val)
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Error::Serde(val)
    }
}

#[derive(Debug, Clone)]
pub struct Ticket {
    pub username: String,
    pub characters: Vec<String>,
    pub ticket: String,
    pub json: serde_json::Value,
}

impl Ticket {
    pub fn request(username: &str, password: &str) -> Result<Ticket, Error> {
        let client = reqwest::Client::new()?;
        let form_contents = [("account", username), ("password", password)];
        let mut response = client
            .post("https://www.f-list.net/json/getApiTicket.php")?
            .form(&form_contents)?
            .send()?;
        let mut response_string = String::new();
        response.read_to_string(&mut response_string)?;

        let json_response: serde_json::Value = serde_json::from_str(&response_string)?;

        if let Some(error) = json_response.get("error").and_then(|x| x.as_str()) {
            if error != "" {
                return Err(Error::Fchat(String::from(error)));
            }
        } else {
            return Err(Error::Fchat(
                format!("Unexpected JSON response: {:?}", json_response),
            ));
        }

        let characters = if let Some(characters) = json_response.get("characters") {
            serde_json::from_value::<Vec<String>>(characters.clone())?
        } else {
            return Err(Error::Fchat(
                String::from(r#"Response didn't contain "characters""#),
            ));
        };

        let ticket = if let Some(ticket) = json_response.get("ticket").and_then(|x| x.as_str()) {
            String::from(ticket)
        } else {
            return Err(Error::Fchat(
                String::from(r#"Response didn't contain "ticket""#),
            ));
        };

        Ok(Ticket {
            username: String::from(username),
            characters: characters,
            ticket: ticket,
            json: json_response,
        })
    }

    pub fn characters(&self) -> &[String] {
        &self.characters
    }

    pub fn ticket(&self) -> String {
        self.ticket.clone()
    }
}
