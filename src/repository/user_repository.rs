use dashmap::DashMap;
use sqlx::PgPool;

use crate::models::User;

#[allow(dead_code)]
pub struct UserRepository {
    users: DashMap<String, User>,
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            users: DashMap::new(),
            pool,
        }
    }

    pub fn create(&mut self, email: String, password: String) -> User {
        let user = User { email: email.clone(), password };
        self.users.insert(email, user.clone());

        user
    }

    pub fn find_one(&mut self, email: String) -> Option<User> {
        Some(self.users.get(&email)?.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    fn test_create_and_find_one(pool: PgPool) {
        let mut ur = UserRepository::new(pool);
        let user = ur.create(
            "asdf@gmail.com".to_string(),
            "HelloWorld123".to_string(),
        );

        let found = ur.find_one(user.email.to_string());
        assert_eq!(found.unwrap().email, "asdf@gmail.com");
    }
}
