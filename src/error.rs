use crate::message::server;
use std::fmt;
use tokio_tungstenite::tungstenite as ws;

#[derive(Debug)]
pub enum Error {
    WebSocket(ws::Error),
    Parse(server::ParseError),
    Channel,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::WebSocket(ref err) => err.fmt(f),
            Error::Parse(ref err) => err.fmt(f),
            Error::Channel => "Error sending message through channel.".fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<server::ParseError> for Error {
    fn from(error: server::ParseError) -> Self {
        Error::Parse(error)
    }
}

impl From<ws::Error> for Error {
    fn from(error: ws::Error) -> Self {
        Error::WebSocket(error)
    }
}
