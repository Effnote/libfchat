#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate url;

pub use ticket::Ticket;
pub use fchat::FChat;

pub mod ticket;
pub mod fchat;
