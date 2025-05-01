use sqlx::{query_as, PgPool};

use crate::models::User;

#[allow(dead_code)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&mut self, email: String, password: String) -> User {
        query_as!(
            User,
            r#"
            INSERT INTO users (email, password)
            VALUES ($1, $2)
            RETURNING email, password
            "#,
            email,
            password
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn find_one(&mut self, email: String) -> Option<User> {
        query_as!(
            User,
            r#"
            SELECT email, password
            FROM users
            WHERE email = $1;
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await
        .ok()
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
        ).await;

        let found = ur.find_one(user.email.to_string()).await;
        assert_eq!(found.unwrap().email, "asdf@gmail.com");
    }
}
