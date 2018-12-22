#[macro_use]
extern crate serde_derive;

pub use crate::fchat::*;
pub use crate::ticket::Ticket;
pub use websocket::futures;

pub mod bbcode;
pub mod enums;
pub mod fchat;
pub mod message;
pub mod ticket;
