use ticket::Ticket;
use message::client;
use message::server;
use enums;

use url::{Url, ParseResult};
use websocket as ws;
use websocket::{Sender, Receiver};
use websocket::result::WebSocketResult as WsResult;

/// Which F-chat server will be connected to.
pub enum Server {
    Normal,
    Debug,
    Other(String),
}

impl Server {
    pub fn url(&self) -> ParseResult<Url> {
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

pub type ClientSender = ws::client::Sender<ws::WebSocketStream>;
pub type ClientReceiver = ws::client::Receiver<ws::WebSocketStream>;

pub struct FChat {
    pub sender: ClientSender,
    pub receiver: ClientReceiver,
}

impl FChat {
    pub fn connect(server: Server) -> Result<FChat, Box<::std::error::Error>> {
        let url = try!(server.url());
        let request = try!(ws::Client::connect(url));
        let response = try!(request.send());
        try!(response.validate());
        let (sender, receiver) = response.begin().split();

        Ok(FChat {
            sender: sender,
            receiver: receiver,
        })
    }

    pub fn identify(&mut self, ticket: &Ticket, character: &str, client_name: &str, client_version: &str) -> ws::result::WebSocketResult<()> {
        let idn = client::Message::IDN {
            method: enums::IdnMethod::Ticket,
            ticket: ticket.ticket(),
            account: &*ticket.username,
            character: character,
            cname: client_name,
            cversion: client_version,
        };
        try!(self.send_raw(&idn.to_string()));
        Ok(())
    }

    pub fn send_raw(&mut self, message: &str) -> ws::result::WebSocketResult<()> {
        let message = ws::Message::text(message);
        self.sender.send_message(&message)
    }

    pub fn incoming_messages<'a>(&'a mut self) -> Box<Iterator<Item = WsResult<Result<server::Message, server::Error>>> + 'a> {
        fn to_message(data: ws::Message) -> Result<server::Message, server::Error> {
            server::Message::from_slice(&data.payload)
        }
        Box::new(self.receiver.incoming_messages().map(|x| x.map(to_message)))
    }
}
