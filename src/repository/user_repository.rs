use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::models::User;

lazy_static! {
    static ref USERS: DashMap<usize, User> = DashMap::new();
}

pub struct UserRepository {
    users: Vec<User>,
}

impl Default for UserRepository {
    fn default() -> Self {
        Self { users: Vec::<User>::new() }
    }
}

impl UserRepository {
    pub fn create(&mut self, email: String) {
        let id = USERS.len().to_string();
        self.users.push(User { id, email });
    }

    pub fn find_one(&mut self, email: String) -> Option<&User> {
        self.users.iter().find(|&u| u.email == email)
    }
}
