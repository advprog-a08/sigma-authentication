use crate::dto::Credentials;
use crate::models::User;
use crate::errors::AuthError;

pub use self::password::PasswordStrategy;
pub use self::google::GoogleStrategy;

mod password;
mod google;

#[rocket::async_trait]
pub trait AuthStrategy: Send + Sync {
    async fn authenticate(&self, creds: Credentials) -> Result<User, AuthError>;
}
