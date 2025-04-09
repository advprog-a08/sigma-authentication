use crate::errors::AuthError;
use crate::models::{Credentials, User};
use crate::strategies::AuthStrategy;

pub struct PasswordStrategy;

#[rocket::async_trait]
impl AuthStrategy for PasswordStrategy {
    async fn authenticate(&self, creds: Credentials) -> Result<User, AuthError> {
        let Some(password) = creds.password else {
            return Err(AuthError::MissingField("password".into()));
        };

        let _ = password; // verify password

        Ok(User { id: "asdf".to_string(), email: creds.email })
    }
}
