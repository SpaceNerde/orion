use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::format;
use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::rc::Rc;
use std::str::{from_utf8, Utf8Error};
use rand::Rng;
use crate::server;

const HELP_MESSAGE: &str = "\n
    Create chat group:  --create-group\n
    Join chat group:    --join-group\n
    Show public groups: --show-groups\n
    Exit the server:    --exit-server\n
";

#[derive(Clone)]
pub struct GroupBook {
    groups: Vec<Group>
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

#[derive(Eq, PartialEq, Debug)]
pub struct Group {
    name: String,
    id: String,
    password: String,
    clients: Vec<TcpStream>
}

impl Clone for Group {
    fn clone(&self) -> Self {
        let mut buffer = Vec::new();

        Group {
            name: self.name.clone(),
            id: self.id.clone(),
            password: self.password.clone(),
            clients: self.clients
                .iter()
                .map(|stream| buffer.push(stream.try_clone()))
                .any(),
        }
    }
}

impl Group {
    pub fn new(password: String, group_book: &GroupBook, name: String) -> Self {
        Group {
            name,
            id: Group::generate_random_seed(group_book),
            password,
            clients: Vec::new(),
        }
    }

    pub fn add_client(&mut self, client: TcpStream) {
        self.clients.push(client);
    }

    pub fn get_clients(&mut self) -> Vec<TcpStream> {
        self.clients
            .iter()
            .map(|stream| stream.try_clone())
            .chain(Vec::new())
            .collect()
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
}

pub fn handle_waiting_connection(mut stream: TcpStream, group_book: &mut GroupBook) {
    let mut data = [0u8; 1200]; // using 120 byte buffer
    stream.set_nonblocking(true).unwrap();
    loop {
        match stream.read(&mut data) {
            Ok(size) if size > 0 => {
                let message =  from_utf8(&data[0..size-2]).unwrap();
                let mut buffer_group = server::connection_handler::Group::new("".to_string(), &group_book, "test".to_string());

                match message {
                    "--help" => {
                        stream.write(HELP_MESSAGE.as_bytes()).expect("TODO: panic message");
                    },
                    "--create-group" => {
                        stream.write("\nyou created a group".as_bytes()).expect("TODO: panic message");
                        buffer_group = server::connection_handler::Group::new("".to_string(), &group_book, "test".to_string());
                        group_book.add_group(&buffer_group);
                    },
                    "--join-group" => {
                        stream.write("\nyou joined a group".as_bytes()).expect("TODO: panic message");
                        buffer_group.add_client(stream.try_clone().unwrap());
                    },
                    "--show-groups" => {
                        stream.write("\nthis are all the groups!".as_bytes()).expect("TODO: panic message");
                        for s in &group_book.groups {
                            stream.write(format!("\n{:?}", s.name).as_bytes()).expect("TODO: panic message");
                        }
                    },
                    "--exit-server" => {
                        stream.shutdown(Shutdown::Both).unwrap();
                    },
                    _ => {}
                }
            },
            Ok(_) => {

            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Do nothing on WouldBlock, just continue the loop
            },
            Err(e) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                println!("Error: {:?}", e);
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

pub fn handle_group_connection() {

}