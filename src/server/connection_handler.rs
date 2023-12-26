
use std::collections::HashMap;

use std::{io, thread};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

use std::str::{from_utf8};
use std::sync::{Arc, Mutex};
use rand::Rng;
use crate::server;

type ClientMap = Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>;


const HELP_MESSAGE: &str = "\n
    Create chat group:  --create-group\n
    Join chat group:    --join-group\n
    Show public groups: --show-groups\n
    Exit the server:    --exit-server\n
";

#[derive(Clone)]
pub struct GroupBook {
    pub(crate) groups: Vec<Group>
}

impl GroupBook {
    pub fn new() -> Self {
        GroupBook {
            groups: Vec::new(),
        }
    }

    pub fn add_group(&mut self, group: &Group) {
        self.groups.push(group.clone());
    }
}

#[derive(Clone, Debug)]
pub struct Group {
    name: String,
    id: String,
    password: String,
    clients: ClientMap,
    open: bool
}

impl Group {
    pub fn new(password: String, group_book: &GroupBook, name: String) -> Self {
        Group {
            name,
            id: Group::generate_random_seed(group_book),
            password,
            clients: ClientMap::new(Mutex::new(HashMap::new())),
            open: false,
        }
    }

    pub fn add_client(&mut self, client: &TcpStream) {
        let mut clients = self.clients.lock().unwrap();
        let len = clients.len();
        clients.insert(len, Arc::new(Mutex::new(client.try_clone().unwrap())));
    }

    pub fn get_clients(&mut self)  {

    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    fn id_exists(id: String, group_book: &GroupBook) -> bool {
        group_book
            .groups.iter()
            .any(|g| g.id == id)
    }

    fn generate_random_seed(group_book: &GroupBook) -> String {
        let charset: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();

        loop {
            let random_string: String = (0..32)
                .map(|_| {
                    let index = rng.gen_range(0..charset.len());
                    charset[index] as char
                })

                .collect();

            if !Group::id_exists(random_string.clone(), group_book) {
                return random_string;
            }
        }

    }

    pub fn change_status(&mut self) {
        self.open = !self.open
    }

    pub fn get_status(&self) -> bool {
        self.open
    }
}

pub fn handle_client(mut stream: TcpStream, group_book: &mut GroupBook) {
    let mut data = [0u8; 1200]; // using 120 byte buffer
    let mut buffer_group = server::connection_handler::Group::new("".to_string(), &group_book, "test".to_string());
    stream.set_nonblocking(true).unwrap();
    while match stream.read(&mut data) {
        Ok(size) if size > 0 => {
            let message =  from_utf8(&data[0..size-2]).unwrap();
            match message {
                "--help" => {
                    stream.write(HELP_MESSAGE.as_bytes()).expect("TODO: panic message");
                    true
                },
                "--create-group" => {
                    stream.write("\nyou created a group".as_bytes()).expect("TODO: panic message");
                    buffer_group = server::connection_handler::Group::new("".to_string(), &group_book, "test".to_string());
                    group_book.add_group(&buffer_group);
                    true
                },
                "--join-group" => {
                    stream.write("\nyou joined a group".as_bytes()).expect("TODO: panic message");
                    buffer_group.add_client(&stream.try_clone().unwrap());
                    false
                },
                "--show-groups" => {
                    stream.write("\nthis are all the groups!".as_bytes()).expect("TODO: panic message");
                    for s in &group_book.groups {
                        stream.write(format!("\n{:?}", s.name).as_bytes()).expect("TODO: panic message");
                    }
                    true
                },
                "--exit-server" => {
                    stream.shutdown(Shutdown::Both).unwrap();
                    false
                },
                _ => {
                    true
                }
            }
        },
        Ok(_) => {
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
    loop {
        let streams = buffer_group.clients.lock().unwrap();
        match stream.read(&mut data) {
            Ok(size) if size > 0 => {
                for (_, stream) in streams.iter() {
                    let mut locked_stream = stream.lock().unwrap();
                    locked_stream.write(&data[0..size-2]).unwrap();
                }
            },
            Ok(_) => {},
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Do nothing on WouldBlock, just continue the loop
            },
            Err(e) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                println!("Error: {:?}", e);
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

pub fn handle_groups(group_book: GroupBook) {
    thread::spawn(move || {
        loop {
            for group in &group_book.groups {
                if !group.open {
                    thread::spawn(move || {
                        loop {
                            println!("test!");
                        }
                    });
                }
            }
        }
    });
}