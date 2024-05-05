use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::str;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    println!("{:?} just connected", stream.peer_addr());

                    let mut reader = BufReader::new(&stream);
                    let mut buffer = String::new();

                    while reader.read_line(&mut buffer).unwrap() > 0 {
                        println!("{:?}", buffer.trim_end());
                        buffer.clear();
                    }
                });
            }
            Err(e) => {

            }
        }
    }

    Ok(())
}
