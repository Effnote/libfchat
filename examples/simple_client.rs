extern crate fchat;
extern crate tokio_core;

use std::io;
use std::io::prelude::*;

use fchat::{Server, Ticket};
use fchat::futures::{Future, Stream};

use tokio_core::reactor::Core;

fn read_line() -> io::Result<String> {
    io::stdout().flush()?;
    let mut string = String::new();
    io::stdin().read_line(&mut string)?;
    // Trim newline from the end
    if let Some(c) = string.pop() {
        if c != '\n' {
            string.push(c);
            return Ok(string);
        }
    }
    if let Some(c) = string.pop() {
        if c != '\r' {
            string.push(c);
        }
    }
    Ok(string)
}

fn main() {
    print!("Username: ");
    let username = read_line().unwrap();
    print!("Password: ");
    let password = read_line().unwrap();
    let ticket = Ticket::request(&username, &password).unwrap();
    let characters = ticket.characters();
    println!("Characters:");
    for (i, character) in characters.iter().enumerate() {
        println!("({}) {}", i, character);
    }
    let character;
    loop {
        print!("Character number: ");
        let input = read_line().unwrap();
        if let Ok(n) = input.parse::<usize>() {
            if n < characters.len() {
                character = characters[n].clone();
                break;
            }
        }
        println!("Not a valid number: {}", input);
    }
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let chat = fchat::connect(Server::Debug, handle)
        .and_then(|(sink, stream)| {
            (
                fchat::identify(
                    sink,
                    &ticket,
                    character,
                    "Simple Test Client".to_owned(),
                    "0.0.1".to_owned(),
                ),
                Ok(stream),
            )
        })
        .and_then(|(_sink, stream)| {
            stream.for_each(|message| Ok(println!("{:?}", message)))
        });
    core.run(chat).unwrap();
}
