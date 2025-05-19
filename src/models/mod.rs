use std::borrow::Cow;

use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Admin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AdminCreate {
    #[validate(email(message = "Email must be valid"))]
    pub email: String,

    #[validate(custom(function = "validate_password"))]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let min_length = 8;
    let err = ValidationError::new("password");

    if password.len() < min_length {
        return Err(err.with_message(Cow::Borrowed("Password must be at least 8 characters long")));
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(err.with_message(Cow::Borrowed("Password must contain at least one uppercase letter")));
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(err.with_message(Cow::Borrowed("Password must contain at least one lowercase letter")));
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(err.with_message(Cow::Borrowed("Password must contain at least one digit")));
    }

    if !password.chars().any(|c| "!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?".contains(c)) {
        return Err(err.with_message(Cow::Borrowed("Password must contain at least one special character")));
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TableSession {
    pub id: Uuid,
    pub table_id: Uuid,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
