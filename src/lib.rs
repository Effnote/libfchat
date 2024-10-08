#[macro_use]
extern crate serde_derive;

pub use crate::connection::Connection;
pub use crate::error::Error;
pub use crate::message::{ClientMessage, ServerMessage};
pub use crate::server::Server;
pub use crate::ticket::Ticket;

pub type Result<T> = std::result::Result<T, Error>;

pub mod bbcode;
pub mod connection;
pub mod enums;
mod error;
pub mod message;
mod server;
pub mod ticket;
