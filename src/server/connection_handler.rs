use std::cell::RefCell;
use std::collections::HashSet;
use rand::Rng;

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



