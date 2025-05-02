use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::repository::UserRepository;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn authenticate(&mut self, email: String, password: String) -> bool {
        let user = self.repo.find_one(email).await;

        if let Some(u) = user {
            let hashed = PasswordHash::new(&u.password).unwrap();
            Argon2::default().verify_password(password.as_bytes(), &hashed).is_ok()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::UserRepository;
    use crate::database::setup_test_db;

    use super::UserService;

    const EMAIL: &str = "asdf@gmail.com";
    const PASSWORD: &str = "helloworld123";

    #[rocket::async_test]
    async fn test_authenticate_correct() {
        let email = EMAIL.to_string();
        let password = PASSWORD.to_string();
        let test_db = setup_test_db().await;

        let mut repo = UserRepository::new(test_db.pool);
        repo.create(email.clone(), password.clone()).await;

        let mut serv = UserService::new(repo);
        let result = serv.authenticate(email, password).await;

        assert!(result);
    }

    #[rocket::async_test]
    async fn test_authenticate_incorrect() {
        let email = EMAIL.to_string();
        let password = PASSWORD.to_string();
        let wrong_password = "asdf".to_string();
        let test_db = setup_test_db().await;

        let mut repo = UserRepository::new(test_db.pool);
        repo.create(email.clone(), password.clone()).await;

        let mut serv = UserService::new(repo);
        let result = serv.authenticate(email, wrong_password).await;

        assert!(!result);
    }
}
