use ticket::Ticket;
use message::client;
use enums;

use url;
use url::{Url, ParseResult};
use websocket;
use std::sync::mpsc::{Sender, Receiver, channel, SendError};

/// Which F-chat server will be connected to.
pub enum Server {
    Normal,
    Debug,
    Other(String),
}

impl Server {
    fn url(&self) -> ParseResult<Url> {
        let string = {
            use self::Server::*;
            match *self {
                Normal => "wss://chat.f-list.net:9799",
                Debug => "wss://chat.f-list.net:8799",
                Other(ref string) => &**string,
            }};
        Url::parse(string)
    }
}

pub struct FChat {
    sender: Sender<String>,
    receiver: Receiver<String>,
    connected: bool,
}

impl FChat {
    pub fn connect(server: Server) -> Result<FChat, Box<::std::error::Error>> {
        let (in_tx, in_rx) = channel();
        let (out_tx, out_rx) = channel();
        Ok(FChat {
            sender: out_tx,
            receiver: in_rx,
            connected: true
        })
    }

    pub fn identify(&mut self, ticket: &Ticket, character: &str, client_name: &str, client_version: &str) {
        let idn = client::Message::IDN {
            method: enums::IdnMethod::Ticket,
            ticket: ticket.ticket(),
            account: &*ticket.username,
            character: character,
            cname: client_name,
            cversion: client_version,
        };
        self.send_raw(idn.to_string());
    }

    pub fn send_raw(&mut self, mut message: String) -> Result<(), SendError<String>> {
        message.push('\n');
        self.sender.send(message)
    }
}
