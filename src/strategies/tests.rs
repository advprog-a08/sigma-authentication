use crate::dto::Credentials;
use crate::errors::AuthError;
use crate::strategies::{AuthStrategy, PasswordStrategy, GoogleStrategy};

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

#[rocket::async_test]
async fn test_google_strategy_success() {
    let strategy = GoogleStrategy;
    let creds = Credentials {
        strategy: "google".to_string(),
        email: "test@example.com".to_string(),
        password: None,
        token: Some("valid_token".to_string()),
    };

    let result = strategy.authenticate(creds).await;
    assert!(result.is_ok());
    
    let user = result.unwrap();
    assert_eq!(user.email, "user@example.com");
    assert!(!user.id.is_empty());
}

#[rocket::async_test]
async fn test_google_strategy_missing_token() {
    let strategy = GoogleStrategy;
    let creds = Credentials {
        strategy: "google".to_string(),
        email: "test@example.com".to_string(),
        password: None,
        token: None,
    };

    let result = strategy.authenticate(creds).await;
    assert!(result.is_err());
    
    match result.unwrap_err() {
        AuthError::MissingField(field) => assert_eq!(field, "token"),
        _ => panic!("Expected MissingField error"),
    }
}
