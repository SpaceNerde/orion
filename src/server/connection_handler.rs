use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
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
    groups: HashSet<String>
}

impl GroupBook {
    pub fn new() -> Self {
        GroupBook {
            groups: HashSet::new(),
        }
    }

    pub fn add_group(&mut self, group: &Group) {
        self.groups.insert(group.get_id());
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Group {
    id: String,
    password: String,
    clients: RefCell<HashSet<String>>
}

impl Group {
    pub fn new(password: String, group_book: &GroupBook) -> Self {
        Group {
            id: Group::generate_random_seed(group_book),
            password,
            clients: RefCell::new(HashSet::new()),
        }
    }

    pub fn add_client(&mut self, client_ip: String) {
        self.clients.borrow_mut().insert(client_ip);
    }

    pub fn get_clients(&mut self) -> HashSet<String> {
        self.clients.get_mut().clone()
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    fn id_exists(id: String, group_book: &GroupBook) -> bool {
        group_book
            .groups.iter()
            .any(|g| g == &id)
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

pub fn handle_waiting_connection(mut stream: TcpStream, group_book: &GroupBook) {
    let mut data = [0u8; 1200]; // using 120 byte buffer
    stream.set_nonblocking(true).unwrap();
    loop {
        match stream.read(&mut data) {
            Ok(size) if size > 0 => {
                let message =  from_utf8(&data[0..size-2]).unwrap();

                match message {
                    "--help" => {
                        stream.write(HELP_MESSAGE.as_bytes()).expect("TODO: panic message");
                    },
                    "--create-group" => {
                        stream.write("\nyou created a group".as_bytes()).expect("TODO: panic message");
                        let mut test_group = server::connection_handler::Group::new("".to_string(), &group_book);
                    },
                    "--join-group" => {
                        stream.write("\nyou joined a group".as_bytes()).expect("TODO: panic message");
                    },
                    "--show-groups" => {
                        stream.write("\nthis are all the groups!".as_bytes()).expect("TODO: panic message");
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