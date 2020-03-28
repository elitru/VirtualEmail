use crate::models::models::{Message, User};
use std::collections::{HashMap, HashSet};

const KEY_LENGTH: i8 = 16;

pub struct Session {
    pub users: HashMap<String, User>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) -> String {
        let values: HashSet<&User> = self.users.values().collect();
        if values.contains(&user) {
            for (key, val) in self.users.iter() {
                if val.email == user.email {
                    return key.to_owned();
                }
            }
        }

        let key = generate_key();
        self.users.insert(key.clone(), user);
        return key;
    }

    pub fn remove_user(&mut self, key: &str) {
        self.users.remove(key);
    }

    pub fn get_user_by_key(&self, key: &str) -> Option<&User> {
        self.users.get(key)
    }
}

fn generate_key() -> String {
    let mut result = String::new();

    for _ in (0..KEY_LENGTH) {
        result.push(rand::random::<char>());
    }

    result
}
