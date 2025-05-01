use crate::repository::UserRepository;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub fn authenticate(&mut self, email: String, password: String) -> bool {
        let user = self.repo.find_one(email);

        if let Some(u) = user {
            u.password == password // temporary implementation
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::UserRepository;

    use super::UserService;

    const EMAIL: &str = "asdf@gmail.com";
    const PASSWORD: &str = "helloworld123";

    #[test]
    fn test_authenticate_correct() {
        let email = EMAIL.to_string();
        let password = PASSWORD.to_string();

        let mut repo = UserRepository::default();
        repo.create(email.clone(), password.clone());

        let mut serv = UserService::new(repo);
        let result = serv.authenticate(email, password);

        assert!(result);
    }

    #[test]
    fn test_authenticate_incorrect() {
        let email = EMAIL.to_string();
        let password = PASSWORD.to_string();
        let wrong_password = "asdf".to_string();

        let mut repo = UserRepository::default();
        repo.create(email.clone(), password.clone());

        let mut serv = UserService::new(repo);
        let result = serv.authenticate(email, wrong_password);

        assert!(!result);
    }
}
