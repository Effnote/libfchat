use hyper;
use url::form_urlencoded::Serializer;
use serde_json;
use std::io::Read;

#[derive(Debug)]
pub enum Error {
    StdIo(::std::io::Error),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    Fchat(String),
    Other
}

impl From<::std::io::Error> for Error {
    fn from(val: ::std::io::Error) -> Self {
        Error::StdIo(val)
    }
}

impl From<hyper::Error> for Error {
    fn from(val: hyper::Error) -> Self {
        Error::Hyper(val)
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Error::Serde(val)
    }
}

// Mime from hyper seems to use Result<Mime, ()>
impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::Other
    }
}

#[derive(Debug)]
pub struct Ticket {
    pub username: String,
    pub characters: Vec<String>,
    pub ticket: String,
    pub json: serde_json::Value,
}

impl Ticket {
    pub fn request(username: &str, password: &str) -> Result<Ticket, Error> {

        let client = hyper::Client::new();

        let body = Serializer::new(String::new())
            .append_pair("account", username)
            .append_pair("password", password)
            .finish();

        let mime = try!("application/x-www-form-urlencoded".parse());

        let mut response = try!(
            client.post("http://www.f-list.net/json/getApiTicket.php")
            .body(&body)
            .header(hyper::header::ContentType(mime))
            .send()
        );

        let mut response_string = String::new();
        try!(response.read_to_string(&mut response_string));

        let json_response: serde_json::Value = try!(serde_json::from_str(&response_string));

        if let Some(&serde_json::Value::String(ref error)) = json_response.find("error") {
            if error != "" {
                return Err(Error::Fchat(error.clone()));
            }
        } else {
            return Err(Error::Fchat(format!("Unexpected JSON response: {:?}", json_response)));
        }

        let characters =
            if let Some(characters) = json_response.find("characters") {
                try!(serde_json::from_value::<Vec<String>>(characters.clone()))
            } else {
                return Err(Error::Fchat(String::from(r#"Response didn't contain "characters""#)))
            };

        let ticket =
            if let Some(&serde_json::Value::String(ref ticket)) = json_response.find("ticket") {
                ticket.clone()
            } else {
                return Err(Error::Fchat(String::from(r#"Response didn't contain "ticket""#)))
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

    pub fn ticket(&self) -> &str {
        &self.ticket
    }
}
