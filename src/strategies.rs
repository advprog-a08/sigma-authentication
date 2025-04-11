use crate::models::{Credentials, User};
use crate::errors::AuthError;

pub use self::password::PasswordStrategy;
pub use self::google::GoogleStrategy;

mod password;
mod google;

#[cfg(test)]
mod tests;

#[rocket::async_trait]
pub trait AuthStrategy: Send + Sync {
    async fn authenticate(&self, creds: Credentials) -> Result<User, AuthError>;
}
