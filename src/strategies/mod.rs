use crate::dto::Credentials;
use crate::models::User;
use crate::errors::AuthError;

pub use self::password::PasswordStrategy;

mod password;

#[rocket::async_trait]
pub trait AuthStrategy: Send + Sync {
    async fn authenticate(&self, creds: Credentials) -> Result<User, AuthError>;
}
