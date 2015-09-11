extern crate fchat;

use std::io;
use fchat::{Ticket, FChat};

fn read_line() -> io::Result<String> {
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
    let ticket = Ticket::new(username, password);
    print!("Characters: ");
    for (i, character) in ticket.characters().enumerate() {
        print!("({}) {}", i, character);
    }
    let character_number;
    loop {
        print!("Character number: ");
        let input = read_line().unwrap();
        if let Some(n) = input.parse::<u32>() {
            character_number = n;
            break;
        } else {
            println!("Not a valid number: {}", input);
        }
    }
}
