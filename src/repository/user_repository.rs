use crate::models::User;

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
        self.users.push(User { id: "1".to_string(), email });
    }

    pub fn find_one(&mut self, email: String) -> Option<&User> {
        self.users.iter().find(|&u| u.email == email)
    }
}
