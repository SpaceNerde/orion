use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// propper disconnect and error handling
//
// Fuck it NO ERROR HANDLING ON MY WATCH!!!

struct Group {}

struct Client {
    stream: TcpStream,
    username: String,
}

impl Client {
    fn new(stream: TcpStream, username: String) -> Self {
        Self { stream, username }
    }

    // To lazy to work on a state system so i just straight up drop the stream to disconnect :|
    fn disconnect(self) {
        drop(self);
    }

    fn send_message(&mut self, message: String) {
        self.stream
            .write(format!("{}: {}", self.username, message).as_bytes())
            .expect("Could not write to client");
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // Create list of all connected clients
    // TODO: Make Fucking Groups and then make this code a litte more clean
    let clients: Vec<Client> = vec![];
    let shared_clients = Arc::new(Mutex::new(clients));
    let shared_clients_thread = Arc::clone(&shared_clients);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let stream_clone = stream.try_clone();

                    // Add client to List of clients
                    // TODO: Change this shit into clean groups later aswell!!
                    let mut clients = shared_clients.lock().unwrap();

                    // create new client
                    let client = Client::new(
                        stream_clone.unwrap(),
                        "TEST USER".to_string(), // TODO
                    );

                    clients.push(client);

                    let tx_clone = tx.clone();

                    thread::spawn(move || {
                        println!("{:?} just connected", stream.peer_addr());

                        let mut reader = BufReader::new(&stream);
                        let mut buffer = String::new();

                        loop {
                            match reader.read_line(&mut buffer) {
                                Ok(_) => {
                                    let message = buffer.to_string();
                                    tx_clone.send(message).unwrap();
                                    buffer.clear();
                                }
                                Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                                    println!("INFO: Client just disconnected.");
                                    break;
                                }
                                Err(e) if e.kind() == ErrorKind::ConnectionReset => {
                                    println!("INFO: Client reset connection.");
                                    break;
                                }
                                Err(e) => {
                                    panic!("ERROR: {:?}", e);
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    panic!("ERROR: {:?}", e);
                }
            }
        }
    });

    loop {
        let received = rx.recv().unwrap();
        let mut clients = shared_clients_thread.lock().unwrap();
        for client in clients.iter_mut() {
            client.send_message(received.clone());
        }
    }
}
