use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::{net::TcpListener, result};

use rand::distr::Alphanumeric;
use rand::{rng, Rng};

type Result<T> = result::Result<T, ()>;

enum Message<'client> {
    Connect(Client<'client>),
    Disconnect(Client<'client>),
    New {
        sender: Client<'client>,
        msg: Vec<u8>,
    },
}

#[derive(Debug, Clone)]
struct Client<'name> {
    stream: Arc<TcpStream>,
    username: &'name str,
    address: SocketAddr,
}

impl<'name> Client<'name> {
    fn new(stream: Arc<TcpStream>, username: &'name str, address: SocketAddr) -> Self {
        Client {
            stream,
            username,
            address,
        }
    }
}

#[derive(Clone, Debug)]
struct Group<'name> {
    clients: HashMap<SocketAddr, Client<'name>>,
    group_name: &'name str,
    group_id: String,
}

impl<'name> Group<'name> {
    fn new(group_name: &'name str) -> Self {
        // Creates 16 character long random string
        let group_id: String = rng()
            .sample_iter(Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        Group {
            clients: HashMap::new(),
            group_name,
            group_id,
        }
    }

    fn insert(&mut self, client: Client<'name>) {
        self.clients.insert(client.address, client);
    }

    fn remove(&mut self, client: Client<'name>) {
        self.clients.remove(&client.address);
    }

    fn send(&self, msg: Vec<u8>, sender: Client) -> Result<()> {
        for client in self.clients.clone() {
            if client.0 == sender.address {
                continue;
            }

            client.1.stream.as_ref().write(&msg).map_err(|e| {
                eprintln!("ERROR: Could not write message to stream: {e}");
            })?;
        }

        Ok(())
    }
}

fn handle_server(rx: Receiver<Message>) -> Result<()> {
    // all clients connected to server
    let mut group = Group::new("sample");

    loop {
        let msg = rx.recv().map_err(|e| {
            eprintln!("ERROR: Could not receive message: {e}");
        })?;

        match msg {
            Message::Connect(client) => {
                // add new connection to list of connected clients
                group.insert(client.clone());

                client
                    .stream
                    .as_ref()
                    .write_all(b"\nWelcome to Orion!\n")
                    .expect("BROKEN");
            }
            Message::Disconnect(client) => {
                group.remove(client.clone());

                println!("INFO: Client disconnected: {:?}", client.address);
            }
            Message::New { sender, msg } => {
                // send message to all connected clients
                group.send(msg, sender).expect("FIXME");
            }
        }
    }
}

fn handle_connection(stream: Arc<TcpStream>, tx: Sender<Message>) -> Result<()> {
    // TODO: Connection message to server must contain token and username

    // create new client instance
    let address = stream.as_ref().peer_addr().map_err(|e| {
        eprintln!("ERROR: Could not get address from stream: {e}");
    })?;

    let client = Client::new(stream.clone(), "john", address);

    tx.send(Message::Connect(client.clone())).map_err(|e| {
        eprintln!("ERROR: Client could not Connect to server: {e}");
    })?;

    // hold connection to server open
    let mut buf = vec![0; 64];
    loop {
        let n = stream.as_ref().read(&mut buf).map_err(|e| {
            eprintln!("ERROR: Could not read from stream: {e}");
            let _ = tx.send(Message::Disconnect(client.clone()));
        })?;

        tx.send(Message::New {
            sender: client.clone(),
            msg: buf[0..n].to_vec(),
        })
        .map_err(|e| {
            eprintln!("ERROR: Could not send message over channel: {e}");
        })?;

        // close connection if connection was closed by client
        if n == 0 {
            let _ = tx.send(Message::Disconnect(client.clone()));
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("INFO: Starting Server");

    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).map_err(|e| {
        eprintln!("ERROR: Could not bind {addr} to server: {e}");
    })?;

    println!("INFO: Server listening on {addr}");

    let (tx, rx) = channel::<Message>();

    // running server handler
    thread::spawn(|| {
        let _ = handle_server(rx);
    });

    // accept all incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let tx = tx.clone();
                let stream = Arc::new(stream);
                thread::spawn(|| {
                    let _ = handle_connection(stream, tx);
                });
            }
            Err(e) => {
                eprintln!("ERROR: Could not accept connection: {e}");
            }
        }
    }

    Ok(())
}
