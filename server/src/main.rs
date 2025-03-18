use std::net::TcpStream;
use std::thread;
use std::{net::TcpListener, result};
use std::io::Write;

type Result<T> = result::Result<T, ()>;

fn handle_connection(mut stream: TcpStream) -> Result<()>{
    println!("INFO: Client Connected to Server: {}", stream.peer_addr().unwrap());
    
    // welcome message
    drop(stream.write_all(b"Welcome To Orion!"));
    
    // hold connection to server open
    std::thread::sleep(std::time::Duration::new(9999999999999, 0));

    Ok(())
}

fn main() -> Result<()> {
    println!("INFO: Starting Server");
    
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).map_err(|e| {
        eprintln!("ERROR: Could not bind {addr} to server: {e}");
    })?;

    println!("INFO: Server listening on {addr}");

    // accept all incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    let _ = handle_connection(stream);
                });
            },
            Err(e) => {
                eprintln!("ERROR: Could not accept connection: {e}");
            },
        }
    }

    Ok(())
}
