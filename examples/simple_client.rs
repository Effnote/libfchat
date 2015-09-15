extern crate fchat;

use std::io;
use std::io::prelude::*;
use fchat::{Ticket, FChat, Server};

fn read_line() -> io::Result<String> {
    try!(io::stdout().flush());
    let mut string = String::new();
    try!(io::stdin().read_line(&mut string));
    // Trim newline from the end
    if let Some(c) = string.pop() {
        if c != '\n' {
            string.push(c);
            return Ok(string)
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
                drop(characters);
                break;
            }
        }
        println!("Not a valid number: {}", input);
    }
    let mut chat = FChat::connect(Server::Debug).unwrap();
    chat.identify(&ticket, &character, "Simple Test Client", "0.0.1");
}
