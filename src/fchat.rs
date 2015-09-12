use ticket::Ticket;
use url;
use url::{Url, ParseResult};
use websocket;

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
    tmp: ()
}

impl FChat {
    pub fn connect(server: Server) -> FChat {
        FChat {
            tmp: ()
        }
    }

    pub fn identify(&mut self, ticket: &Ticket, character: &str, client_ver: &str) {
    }
}
