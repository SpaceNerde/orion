use std::net::{TcpStream};
use std::io::{Read, Write};
use std::{io, thread};
use std::str::from_utf8;
use std::time::Duration;

fn main() {
    let mut stream = match TcpStream::connect("localhost:3333") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect: {}", e);
            return;
        }
    };

    stream.set_nonblocking(true).expect("Failed to set nonblocking mode");

    println!("Successfully connected to server on port 3333");

    let mut clone_stream = stream.try_clone().expect("Failed to clone stream");

    thread::spawn(move || {
        loop {
            let mut message = String::new();
            io::stdin().read_line(&mut message).expect("Could not read message!");
            clone_stream.write(&message.as_bytes()).expect("Could not send message to server!");
        }
    });

    let mut data = [0u8; 1200]; // using a 1200-byte buffer
    loop {
        match stream.read(&mut data) {
            Ok(size) if size > 0 => {
                println!("Received some data: {:?}", from_utf8(&data[0..size]));
            },
            Ok(_) => {
                // Do nothing if receiving no data
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Do nothing on WouldBlock, just continue the loop
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                // Do nothing on timeout, just continue the loop
            },
            Err(ref e) if e.kind() == io::ErrorKind::ConnectionReset => {
                println!("Connection reset by the remote host. Attempting to reconnect...");
                // Add your logic to handle the connection reset and possibly attempt to reconnect.
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}