use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::{net::TcpListener, result};

type Result<T> = result::Result<T, ()>;

enum Message {
    Connect(Arc<TcpStream>),
    Disconnect(Arc<TcpStream>),
    New {
        sender: Arc<TcpStream>,
        msg: Vec<u8>,
    },
}

fn handle_server(rx: Receiver<Message>) -> Result<()> {
    // all clients connected to server
    let mut client: HashMap<SocketAddr, Arc<TcpStream>> = HashMap::new();

    loop {
        let msg = rx.recv().map_err(|e| {
            eprintln!("ERROR: Could not receive message: {e}");
        })?;

        match msg {
            Message::Connect(stream) => {
                // add new connection to list of connected clients
                client.insert(stream.peer_addr().expect("BROKEN"), stream.clone());

                stream
                    .as_ref()
                    .write_all(b"\nWelcome to Orion!\n")
                    .expect("BROKEN");
            }
            Message::Disconnect(stream) => {
                client.remove(&stream.peer_addr().expect("BROKEN"));

                println!(
                    "INFO: Client disconnected: {:?}",
                    stream.peer_addr().unwrap()
                );
            }
            Message::New { sender, msg } => {
                // send message to all connected clients
                for (addr, client) in client.iter() {
                    if sender.as_ref().peer_addr().expect("BROKEN") == *addr {
                        continue;
                    }
                    let _ = client.as_ref().write(&msg.to_vec());
                }
            }
        }
    }
}

fn handle_connection(stream: Arc<TcpStream>, tx: Sender<Message>) -> Result<()> {
    tx.send(Message::Connect(stream.clone())).map_err(|e| {
        eprintln!("ERROR: Client could not Connect to server: {e}");
    })?;

    // hold connection to server open
    let mut buf = vec![0; 64];
    loop {
        let n = stream.as_ref().read(&mut buf).map_err(|e| {
            eprintln!("ERROR: Could not read from stream: {e}");
            let _ = tx.send(Message::Disconnect(stream.clone()));
        })?;

        tx.send(Message::New {
            sender: stream.clone(),
            msg: buf[0..n].to_vec(),
        })
        .map_err(|e| {
            eprintln!("ERROR: Could not send message over channel: {e}");
        })?;

        // close connection if connection was closed by client
        if n == 0 {
            let _ = tx.send(Message::Disconnect(stream.clone()));
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
