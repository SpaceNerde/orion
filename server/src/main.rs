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
use std::io::ErrorKind;

// propper disconnect and error handling
//
// To do:
//  - message gets lost
//  - cant clone stuff
//  - server shutsdown
//  - weird message format?

struct Group {

}

struct Client {
    stream: TcpStream,
    username: String,
}

impl Client {
    fn new(stream: TcpStream, username: String) -> Self {
        Self {
            stream,
            username,
        }
    }
    
    // To lazy to work on a state system so i just straight up drop the stream to disconnect :|
    fn disconnect(&mut self) {
        drop(self);
    }

    fn send_message(&mut self, message: String) {
        self.stream.write(format!("{}: {}", self.username, message).as_bytes()).expect("Could not write to client");
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    
    // Create list of all connected clients
    // TODO: Make Fucking Groups and then make this code a litte more clean
    let mut clients: Vec<Client> = vec![];
    let shared_clients = Arc::new(Mutex::new(clients));
    let shared_clients_thread = Arc::clone(&shared_clients);

    let (tx, rx) = mpsc::channel();
  
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let stream_clone = stream.try_clone();
                    
                    // Add client to List of clients 
                    // TODO: Change this shit into clean groups later aswell!!
                    let mut clients = shared_clients.lock().unwrap();

                    // create new client
                    let mut client = Client::new(
                        stream_clone.unwrap(), 
                        "TEST USER".to_string(),
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
                                },                  
                                Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                                    println!("INFO: Client just disconnected.");
                                    break;
                                },
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

                }
            }
        }
    });


    while let received = rx.recv().unwrap() {
        let mut clients = shared_clients_thread.lock().unwrap();
        for mut client in clients.iter_mut() {
            client.send_message(received.clone());
        }
    }


    Ok(())
}
