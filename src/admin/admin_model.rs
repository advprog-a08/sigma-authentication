use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::proto;

#[derive(Debug, Clone, Serialize)]
pub struct AdminModel {
    pub email: String,
    pub name: String,
    pub password: String,
}

impl Into<proto::Admin> for AdminModel {
    fn into(self) -> proto::Admin {
        proto::Admin { email: self.email }
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct ValidatedCreateAdminRequest {
    #[validate(email(message = "Email must be valid"))]
    pub email: String,

    #[validate(length(max = 255))]
    pub name: String,

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
        return Err(err.with_message(Cow::Borrowed(
            "Password must contain at least one uppercase letter",
        )));
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(err.with_message(Cow::Borrowed(
            "Password must contain at least one lowercase letter",
        )));
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(err.with_message(Cow::Borrowed("Password must contain at least one digit")));
    }

    if !password
        .chars()
        .any(|c| "!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?".contains(c))
    {
        return Err(err.with_message(Cow::Borrowed(
            "Password must contain at least one special character",
        )));
    }

    Ok(())
}

impl TryFrom<proto::CreateAdminRequest> for ValidatedCreateAdminRequest {
    type Error = tonic::Status;

    fn try_from(value: proto::CreateAdminRequest) -> Result<Self, Self::Error> {
        let v = ValidatedCreateAdminRequest {
            email: value.email,
            name: value.name,
            password: value.password,
        };

        v.validate().map_err(|e| {
            tonic::Status::invalid_argument(format!("Validation failed: {}", e))
        })?;

        Ok(v)
    }
}
