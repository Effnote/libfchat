use crate::enums;
use crate::message::client;
use crate::message::server;
use crate::ticket::Ticket;

use std::convert::From;
use std::fmt;

use tokio::spawn;
use websocket;
use websocket::futures::sync::mpsc::channel;
use websocket::futures::{sink, Future, Sink, Stream};
use websocket::result::WebSocketError;
use websocket::url::{self, Url};
use websocket::OwnedMessage;

/// Which F-chat server will be connected to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Server {
    Normal,
    Debug,
    Other(String),
}

impl Server {
    pub fn url(&self) -> Result<Url, url::ParseError> {
        let string = {
            use self::Server::*;
            match *self {
                Normal => "wss://chat.f-list.net/chat2",
                Debug => "wss://chat.f-list.net/chat2",
                Other(ref string) => &**string,
            }
        };
        Url::parse(string)
    }
}

#[derive(Debug)]
pub enum Error {
    WebSocket(WebSocketError),
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

impl From<WebSocketError> for Error {
    fn from(error: WebSocketError) -> Self {
        Error::WebSocket(error)
    }
}

pub type FchatStream = Box<Stream<Item = server::Message, Error = Error> + Send>;
pub type FchatSink = Box<Sink<SinkItem = client::Message, SinkError = Error> + Send>;
pub type Connection = (FchatSink, FchatStream);

pub fn connect(server: &Server) -> impl Future<Item = Connection, Error = Error> {
    let url = server.url().expect("Invalid server URL provided");
    let future_client = websocket::ClientBuilder::from_url(&url).async_connect_secure(None);
    future_client
        .map_err(Error::WebSocket)
        .and_then(move |(client, _headers)| {
            let (sink, stream) = client.split();
            Ok(wrap(sink, stream))
        })
}

fn wrap<A, B>(sink: A, stream: B) -> Connection
where
    A: Sink<SinkItem = OwnedMessage, SinkError = WebSocketError> + Send + 'static,
    B: Stream<Item = OwnedMessage, Error = WebSocketError> + Send + 'static,
{
    let (tx, rx) = channel(10);
    let mut tx_clone = Some(tx.clone());
    let stream = stream
        .map_err(Error::WebSocket)
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
    spawn(sink.sink_map_err(|_| ()).send_all(rx).map(|_| ()));
    let tx = tx
        .sink_map_err(|_| Error::Channel)
        .with(|message: client::Message| Ok(OwnedMessage::Text(message.to_string())));
    (Box::new(tx), Box::new(stream))
}

pub fn identify<S>(
    sink: S,
    ticket: &Ticket,
    character: String,
    client_name: String,
    client_version: String,
) -> sink::Send<S>
where
    S: Sink<SinkItem = client::Message, SinkError = Error>,
{
    let idn = client::Message::IDN {
        method: enums::IdnMethod::Ticket,
        ticket: ticket.ticket(),
        account: ticket.username.clone(),
        character,
        cname: client_name,
        cversion: client_version,
    };
    sink.send(idn)
}
