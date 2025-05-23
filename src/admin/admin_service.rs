use argon2::{Argon2, PasswordHash, PasswordVerifier};
use thiserror::Error;

use super::{AdminModel, AdminRepository, AdminRepositoryError};

#[derive(Error, Debug)]
pub enum AdminServiceError {
    #[error("{0}")]
    Repository(#[from] AdminRepositoryError),

    #[error("The provided credentials is incorrect")]
    InvalidCredentials,
}

pub struct AdminService {
    repo: AdminRepository,
}

impl AdminService {
    pub fn new(repo: AdminRepository) -> Self {
        Self { repo }
    }

    pub async fn register_admin(
        &self,
        email: String,
        name: String,
        password: String,
    ) -> Result<AdminModel, AdminServiceError> {
        Ok(self.repo.create(email, name, password).await?)
    }

    pub async fn find_one(&self, email: String) -> Result<Option<AdminModel>, AdminServiceError> {
        Ok(self.repo.find_one(email).await?)
    }

    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> Result<(), AdminServiceError> {
        match self.repo.find_one(email).await? {
            Some(admin) => {
                let hashed = PasswordHash::new(&admin.password).unwrap();
                Argon2::default()
                    .verify_password(password.as_bytes(), &hashed)
                    .map_err(|_| AdminServiceError::InvalidCredentials)
            }
            None => Err(AdminServiceError::InvalidCredentials),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database::setup_test_db;

    use super::{AdminRepository, AdminService};

    const EMAIL: &str = "asdf@gmail.com";
    const NAME: &str = "asdf";
    const PASSWORD: &str = "helloworld123";

    #[tokio::test]
    async fn test_register_admin() {
        let email = EMAIL.to_string();
        let name = NAME.to_string();
        let password = PASSWORD.to_string();
        let test_db = setup_test_db().await;

        let repo = AdminRepository::new(test_db.pool);
        let serv = AdminService::new(repo);

        let result = serv.register_admin(email, name, password).await.unwrap();

        assert_eq!(result.email, EMAIL.to_string());
    }

    #[tokio::test]
    async fn test_authenticate_correct() {
        let email = EMAIL.to_string();
        let name = NAME.to_string();
        let password = PASSWORD.to_string();
        let test_db = setup_test_db().await;

        let repo = AdminRepository::new(test_db.pool);
        repo.create(email.clone(), name.clone(), password.clone()).await.unwrap();

        let serv = AdminService::new(repo);
        let result = serv.authenticate(email, password).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authenticate_incorrect() {
        let email = EMAIL.to_string();
        let name = NAME.to_string();
        let password = PASSWORD.to_string();
        let wrong_password = "asdf".to_string();
        let test_db = setup_test_db().await;

        let repo = AdminRepository::new(test_db.pool);
        repo.create(email.clone(), name.clone(), password.clone()).await.unwrap();

        let serv = AdminService::new(repo);
        let result = serv.authenticate(email, wrong_password).await;

        assert!(result.is_err());
    }
}
