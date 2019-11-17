/// Which F-chat server will be connected to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Server {
    Normal,
    Other(String),
}

impl Server {
    pub fn url(&self) -> Result<url::Url, url::ParseError> {
        let string = {
            use self::Server::*;
            match *self {
                Normal => "wss://chat.f-list.net/chat2",
                Other(ref string) => string,
            }
        };
        url::Url::parse(string)
    }
}
