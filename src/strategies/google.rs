use crate::errors::AuthError;
use crate::models::{Credentials, User};
use crate::strategies::AuthStrategy;

pub struct GoogleStrategy;

#[rocket::async_trait]
impl AuthStrategy for GoogleStrategy {
    async fn authenticate(&self, creds: Credentials) -> Result<User, AuthError> {
        let Some(token) = creds.token else {
            return Err(AuthError::MissingField("token".into()));
        };

        let _ = token; // use token to verify

        Ok(User { id: "asdf".to_string(), email: "user@example.com".into() })
    }
}
