use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHasher, SaltString};
use sqlx::{PgPool, query_as};
use thiserror::Error;

use super::AdminModel;

#[derive(Error, Debug)]
pub enum AdminRepositoryError {
    #[error("An error occurred with the database")]
    Database(#[from] sqlx::Error),

    #[error("An error occurred while creating admin")]
    CreateAdmin,
}

#[allow(dead_code)]
pub struct AdminRepository {
    pool: PgPool,
}

impl AdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        email: String,
        name: String,
        password: String,
    ) -> Result<AdminModel, AdminRepositoryError> {
        let salt = SaltString::generate(&mut OsRng);
        let password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AdminRepositoryError::CreateAdmin)?
            .to_string();

        Ok(query_as!(
            AdminModel,
            r#"
            INSERT INTO admins (email, name, password)
            VALUES ($1, $2, $3)
            RETURNING email, name, password
            "#,
            email,
            name,
            password
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn find_one(
        &self,
        email: String,
    ) -> Result<Option<AdminModel>, AdminRepositoryError> {
        Ok(query_as!(
            AdminModel,
            r#"
            SELECT email, name, password
            FROM admins
            WHERE email = $1;
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::setup_test_db;

    use super::*;

    #[tokio::test]
    async fn test_hash_password() {
        let test_db = setup_test_db().await;

        let ar = AdminRepository::new(test_db.pool);
        let admin = ar
            .create(
                "asdf@gmail.com".to_string(),
                "asdf".to_string(),
                "HelloWorld123".to_string(),
            )
            .await
            .unwrap();

        let Some(found) = ar.find_one(admin.email.to_string()).await.unwrap() else {
            panic!()
        };
        assert_ne!(found.password, "HelloWorld123".to_string());
    }

    #[tokio::test]
    async fn test_create_and_find_one() {
        let test_db = setup_test_db().await;

        let ar = AdminRepository::new(test_db.pool);
        let admin = ar
            .create(
                "asdf@gmail.com".to_string(),
                "asdf".to_string(),
                "HelloWorld123".to_string(),
            )
            .await
            .unwrap();

        let Some(found) = ar.find_one(admin.email.to_string()).await.unwrap() else {
            panic!()
        };
        assert_eq!(found.email, "asdf@gmail.com");
    }
}
