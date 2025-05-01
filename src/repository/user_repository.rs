use dashmap::DashMap;

use crate::models::User;

pub struct UserRepository {
    users: DashMap<String, User>,
}

impl Default for UserRepository {
    fn default() -> Self {
        Self { users: DashMap::new() }
    }
}

impl UserRepository {
    pub fn create(&mut self, email: String) -> User {
        let id = self.users.len().to_string();
        let user = User { id: id.clone(), email };
        self.users.insert(id.clone(), user.clone());

        user
    }

    pub fn find_one(&mut self, id: String) -> Option<User> {
        Some(self.users.get(&id)?.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_find_one() {
        let mut ur = UserRepository::default();
        let user = ur.create("asdf@gmail.com".to_string());

        let found = ur.find_one(user.id.to_string());
        assert_eq!(found.unwrap().email, "asdf@gmail.com");
    }
}
