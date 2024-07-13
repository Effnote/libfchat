use tokio_tungstenite::tungstenite::http::{uri::InvalidUri, Uri};

/// Which F-chat server will be connected to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Server {
    Normal,
    Other(String),
}

impl Server {
    pub fn uri(&self) -> Result<Uri, InvalidUri> {
        match *self {
            Server::Normal => "wss://chat.f-list.net/chat2".parse(),
            Server::Other(ref string) => string.parse(),
        }
    }
}
