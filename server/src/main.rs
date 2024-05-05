use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::str;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::mpsc;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    let mut clients: Vec<TcpStream> = vec![];
    let shared_clients = Arc::new(Mutex::new(clients));
    let shared_clients_thread = Arc::clone(&shared_clients);

    let (tx, rx) = mpsc::channel();
  
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let stream_clone = stream.try_clone();

                    let mut clients = shared_clients.lock().unwrap();
                    clients.push(stream_clone.unwrap());

                    let tx_clone = tx.clone();

                    thread::spawn(move || {
                        println!("{:?} just connected", stream.peer_addr());

                        let mut reader = BufReader::new(&stream);
                        let mut buffer = String::new();


                        while reader.read_line(&mut buffer).unwrap() > 0 {
                            let message = buffer.trim_end().to_string();
                            tx_clone.send(message).unwrap();
                            buffer.clear();
                        }
                    });
                }
                Err(e) => {

                }
            }
        }
    });


    while let received = rx.recv() {
        let mut clients = shared_clients_thread.lock().unwrap();
        for mut client in clients.iter() {
            client.write(received.clone().unwrap().as_bytes()).unwrap();
        }
    }


    Ok(())
}
