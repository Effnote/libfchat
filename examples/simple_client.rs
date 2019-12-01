use std::io;
use std::io::prelude::*;

use fchat::futures::prelude::*;

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
    let ticket = fchat::Ticket::request(&username, &password).unwrap();
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
    let chat = async {
        let mut connection = fchat::Connection::connect(&fchat::Server::Normal).await?;
        connection
            .identify(
                &ticket,
                character,
                String::from("Simple Test Client"),
                String::from("0.0.1"),
            )
            .await?;
        connection
            .for_each(|message| {
                async move {
                    println!("{:?}", message);
                }
            })
            .await;
        Ok::<(), fchat::Error>(())
    };
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(chat).unwrap();
}
