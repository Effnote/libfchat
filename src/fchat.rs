use crate::enums;
use crate::message::client;
use crate::message::server;
use crate::ticket::Ticket;

use std::convert::From;
use std::fmt;

use tokio::prelude::*;
use tokio::sync::mpsc::channel;
use tokio_tungstenite as async_ws;
use tokio_tungstenite::{WebSocketStream, tungstenite as ws};

/// Which F-chat server will be connected to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Server {
    Normal,
    Debug,
    Other(String),
}

impl Server {
    pub fn url(&self) -> Result<url::Url, url::ParseError> {
        let string = {
            use self::Server::*;
            match *self {
                Normal => "wss://chat.f-list.net/chat2",
                Debug => "wss://chat.f-list.net/chat2",
                Other(ref string) => &**string,
            }
        };
        url::Url::parse(string)
    }
}

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

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::WebSocket(ref err) => err.description(),
            Error::Parse(ref err) => err.description(),
            Error::Channel => "Error sending message through channel.",
        }
    }
}

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

pub type FchatStream = Box<dyn Stream<Item = server::Message> + Send>;
pub type FchatSink = Box<dyn Sink<client::Message, Error = Error> + Send>;
pub type Connection = (FchatSink, FchatStream);

pub async fn connect(server: &Server) -> Result<Connection, Error> {
    let url = server.url().expect("Invalid server URL provided");
    let (connection, _response) = async_ws::connect_async(url).await?;
    Ok(wrap(connection))
}

fn wrap<S>(stream: WebSocketStream<S>) -> Connection
where
    S: AsyncWrite + AsyncRead + Unpin
{
    let (sink, stream) = stream.split();
    let (tx, rx) = channel(10);
    let mut tx_clone = Some(tx.clone());
    let stream = stream
        .filter_map(move |message| match message {
            OwnedMessage::Text(ref data) => {
                Some(server::Message::from_slice(data.as_bytes()).map_err(Error::Parse))
            }
            OwnedMessage::Binary(ref data) => {
                Some(server::Message::from_slice(data).map_err(Error::Parse))
            }
            OwnedMessage::Ping(data) => {
                tx_clone = tx_clone
                    .take()
                    .map(|tx| tx.send(OwnedMessage::Pong(data)).wait().unwrap());
                None
            }
            _ => None,
        })
        .and_then(|x| x);
    tokio::spawn(sink.sink_map_err(|_| ()).send_all(rx).map(|_| ()));
    let tx = tx
        .sink_map_err(|_| Error::Channel)
        .with(|message: client::Message| Ok(OwnedMessage::Text(message.to_string())));
    (Box::new(tx), Box::new(stream))
}

pub async fn identify<S>(
    sink: S,
    ticket: &Ticket,
    character: String,
    client_name: String,
    client_version: String,
)
where
    S: Sink<client::Message, Error = Error> + Unpin,
{
    let idn = client::Message::IDN {
        method: enums::IdnMethod::Ticket,
        ticket: ticket.ticket(),
        account: ticket.username.clone(),
        character,
        cname: client_name,
        cversion: client_version,
    };
    sink.send(idn).await;
}
