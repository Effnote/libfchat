#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate url;
extern crate websocket;

pub use ticket::Ticket;
pub use fchat::{FChat, Server};

pub mod ticket;
pub mod fchat;
pub mod message;
pub mod enums;
