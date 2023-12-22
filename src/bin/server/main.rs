use std::{io, thread};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0u8; 1200]; // using 120 byte buffer
    stream.set_nonblocking(true).unwrap();
    while match stream.read(&mut data) {
        Ok(size) if size > 0 => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            println!("Server: {:?}", from_utf8(&data[0..size]));
            true
        },
        Ok(_) => {
            // Receive no data and continue
            true
        },
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            // Do nothing on WouldBlock, just continue the loop
            true
        },
        Err(e) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            println!("Error: {:?}", e);
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}