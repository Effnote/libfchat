extern crate fchat;
extern crate serde_json;

use fchat::message::client::Message;
use fchat::enums::*;

fn main() {
    println!("{}", serde_json::to_value(Message::PIN).unwrap());
    println!(
        "{}",
        serde_json::to_value(Message::MSG {
            channel: String::from("foo"),
            message: String::from("bar"),
        }).unwrap()
    );
    println!(
        "{}",
        serde_json::to_value(Message::STA {
            status: CharacterStatus::Online,
            statusmsg: String::from("Yohohoho"),
        }).unwrap()
    );
    println!(
        "{}",
        serde_json::to_value(Message::IGN(IgnEnum::Add {
            character: String::from("foo_bar"),
        })).unwrap()
    );
    println!(
        "{}",
        serde_json::to_value(Message::IGN(IgnEnum::List)).unwrap()
    );

    println!("{:?}", Message::PIN.to_string());
    println!(
        "{:?}",
        Message::MSG {
            channel: String::from("foo"),
            message: String::from("bar"),
        }.to_string()
    );
    println!(
        "{:?}",
        Message::STA {
            status: CharacterStatus::Online,
            statusmsg: String::from("Yohohoho"),
        }.to_string()
    );
    println!(
        "{:?}",
        Message::IGN(IgnEnum::Add {
            character: String::from("foo_bar"),
        }).to_string()
    );
    println!("{:?}", Message::IGN(IgnEnum::List).to_string());
}
