use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// Handles a single client
fn handle_client(mut stream: TcpStream, state: Arc<Mutex<Vec<(String, String)>>>) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);

    let mut buf = [0; 512];
    
    loop {
        let bytes_read = stream.read(&mut buf)?;
        let command = String::from_utf8_lossy(&buf[..bytes_read - 1]);

        let mut state = state.lock().unwrap();

        if bytes_read == 0 {
            return Ok(());
        }
        
        match Request::new(&command) {
            Request::Append(key, value) => {
                let key_present = state.iter().any(|x| x.0 == key);

                if !key_present {
                    state.push((key, value));
                } 

            },
            Request::None => println!("Wrong Command!"),
        }

        stream.write(&buf[..bytes_read - 1])?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");

    let state: Arc<Mutex<Vec<(String,String)>>>  = Arc::new(Mutex::new(Vec::new()));

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

enum Request {
    Append(String,String),
    None,
}

impl Request {
    fn new(command: &str) -> Request  {

        let command = command.trim();
        let command = command.to_lowercase();
        let command: Vec<&str> = command.split_whitespace().collect();

        match &command[..] {
            ["append", key, value] => Request::Append(key.to_string(), value.to_string()),
            _ => Request::None,
        }
    }
}
