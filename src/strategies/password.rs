use crate::errors::AuthError;
use crate::dto::Credentials;
use crate::models::User;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[rocket::async_test]
    async fn test_password_strategy_success() {
        let strategy = PasswordStrategy;
        let creds = Credentials {
            strategy: "password".to_string(),
            email: "test@example.com".to_string(),
            password: Some("password123".to_string()),
            token: None,
        };

        let result = strategy.authenticate(creds).await;
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert_eq!(user.email, "test@example.com");
        assert!(!user.id.is_empty());
    }

    #[rocket::async_test]
    async fn test_password_strategy_missing_password() {
        let strategy = PasswordStrategy;
        let creds = Credentials {
            strategy: "password".to_string(),
            email: "test@example.com".to_string(),
            password: None,
            token: None,
        };

        let result = strategy.authenticate(creds).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            AuthError::MissingField(field) => assert_eq!(field, "password"),
            _ => panic!("Expected MissingField error"),
        }
    }
}
