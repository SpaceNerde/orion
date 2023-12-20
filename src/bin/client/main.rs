use std::net::{TcpStream};
use std::io::{Read, Write};
use std::{io, thread};
use std::io::ErrorKind::TimedOut;
use std::time::Duration;

fn main() {
    thread::spawn(move || {
        match TcpStream::connect("localhost:3333") {
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");

                loop {
                    let mut message = String::new();
                    io::stdin().read_line(&mut message).expect("Could not read message!");
                    println!("{:?}", &message.as_bytes());
                    stream.write(&message.as_bytes()).expect("Could not send message to server!");
                }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    });

    thread::spawn(move || {
        match TcpStream::connect("localhost:3333") {
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");

                let mut data = [0u8; 1200]; // using 1200 byte buffer
                loop  {
                    stream.set_read_timeout(Some(Duration::new(1,0))).expect("Something went wrong");
                    match stream.read(&mut data[..]) {
                        Ok(_) => {
                            println!("Received some data: {:?}", String::from_utf8((&mut data).to_vec()));
                        },
                        Err(ref e) if e.kind() == TimedOut => {},
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                }

            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    });
    loop {

    }
    println!("Terminated.");
}