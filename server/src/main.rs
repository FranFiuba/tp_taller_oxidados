use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

// Handles a single client
fn handle_client(
    mut stream: TcpStream,
    state: Arc<Mutex<Vec<(String, String)>>>,
) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);

    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;

        if bytes_read == 0 {
            return Ok(());
        }

        let command = str::from_utf8(&buf[..bytes_read]).unwrap();
        let mut state = state.lock().unwrap();

        match Command::new(&command) {
            Command::Append(key, value) => {
                let key_present = state.iter().any(|x| x.0 == key);

                if !key_present {
                    state.push((String::from(key), String::from(value)));
                }
                println!("command!")
            }
            Command::None => println!("Wrong Command!"),
        }

        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
  
    let state: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {


        match stream {
            Err(e) => {
                eprintln!("failed: {}", e)
            }
            Ok(stream) => {
                let state = Arc::clone(&state);
                thread::spawn(move || {
                    handle_client(stream, state).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

enum Command<'a> {
    Append(&'a str, &'a str),
    None,
}

impl<'a> Command<'a> {
    fn new(command: &str) -> Command {
        let command: Vec<&str> = command.trim().split_whitespace().collect();

        match &command[..] {
            ["append", key, value] => Command::Append(key, value),
            _ => Command::None,
        }
    }
}
