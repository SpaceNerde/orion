use std::io::{stdin, BufRead, BufReader, ErrorKind, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> std::io::Result<()> {
    match TcpStream::connect("127.0.0.1:80") {
        Ok(mut stream) => {
            // message reciving thread
            let stream_clone = stream.try_clone().unwrap();

            thread::spawn(move || {
                let mut reader = BufReader::new(&stream_clone);
                let mut buffer = String::new();

                loop {
                    match reader.read_line(&mut buffer) {
                        Ok(_) => {
                            let message = buffer.trim_end().to_string();
                            println!("{:?}", message);
                            buffer.clear();
                        }
                        Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                            println!("INFO: Connection to server lost.");
                            break;
                        }
                        Err(e) if e.kind() == ErrorKind::ConnectionReset => {
                            println!("INFO: Connection to server lost.");
                            break;
                        }
                        Err(e) => {
                            panic!("ERROR: {:?}", e);
                        }
                    }
                }
            });

            // message sending loop
            loop {
                let mut buffer = String::new();
                let handle = stdin();

                match handle.read_line(&mut buffer) {
                    Ok(_) => match stream.write(buffer.as_bytes()) {
                        Ok(_) => {}
                        Err(e) if e.kind() == ErrorKind::ConnectionReset => {
                            println!("INFO: There is no connection to the host.");
                            break;
                        }
                        Err(e) => {
                            panic!("ERROR: {:?}", e);
                        }
                    },
                    Err(e) if e.kind() == ErrorKind::ConnectionReset => {
                        println!("INFO: There is no connection to the host.");
                        break;
                    }
                    Err(e) => {
                        panic!("ERROR: {:?}", e);
                    }
                }
            }
        }
        Err(_) => {
            // You dont always have to panic if something goes wrong ;)
            println!("ERROR: Could not connect to server!");
        }
    }

    Ok(())
}
