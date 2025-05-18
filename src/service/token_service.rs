use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}

#[derive(Error, Debug)]
pub enum TokenServiceError {
    #[error("JWT Error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("An error occurred")]
    OtherError,
}

pub struct TokenService {
    service_name: String,
    decoding_key: DecodingKey,
    encoding_key: EncodingKey,
}

impl TokenService {
    pub fn new(service_name: String, secret: String) -> Self {
        let secret = secret.as_bytes();

        Self { 
            service_name,
            decoding_key: DecodingKey::from_secret(secret),
            encoding_key: EncodingKey::from_secret(secret),
        }
    }

    pub fn create_jwt(&self, admin_id: String) -> Result<String, TokenServiceError> {
        let iat = Utc::now().timestamp() as usize;

        let exp = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .ok_or(TokenServiceError::OtherError)?
            .timestamp() as usize;

        let claims = Claims {
            iss: self.service_name.clone(),
            sub: admin_id,
            iat,
            exp,
        };

        Ok(encode(&Header::default(), &claims, &self.encoding_key)?)
    }

    pub fn decode_jwt(&self, token: String) -> Result<Claims, TokenServiceError> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        let token = decode::<Claims>(&token, &self.decoding_key, &validation)?;

        Ok(token.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    const SERVICE_NAME: &str = "sigma";

    fn setup_service() -> TokenService {
        TokenService::new(SERVICE_NAME.to_string(), "test-secret".to_string())
    }

    fn setup_fake_service() -> TokenService {
        TokenService::new("fake".to_string(), "fake".to_string())
    }

    #[test]
    fn test_jwt_roundtrip() {
        let service = setup_service();
        let admin_id = "admin123".to_string();

        let token = service.create_jwt(admin_id.clone()).unwrap();
        let claims = service.decode_jwt(token).unwrap();

        assert_eq!(claims.iss, SERVICE_NAME.to_string());
        assert_eq!(claims.sub, admin_id);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_claims_content() {
        let service = setup_service();
        let admin_id = "alice".to_string();

        let token = service.create_jwt(admin_id.clone()).unwrap();
        let claims = service.decode_jwt(token).unwrap();

        assert_eq!(claims.sub, "alice");
        let now = Utc::now().timestamp() as usize;
        assert!(claims.iat <= now);
        assert!(claims.exp > now);
    }

    #[test]
    fn test_invalid_token() {
        let service = setup_service();

        let invalid_token = "this.is.not.valid".to_string();

        assert!(matches!(service.decode_jwt(invalid_token), Err(TokenServiceError::JwtError(..))));
    }

    #[test]
    fn test_tampered_token() {
        let service = setup_service();
        let token = service.create_jwt("bob".to_string()).unwrap();

        let mut parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);

        parts[1] = "tampered_payload";
        let tampered_token = parts.join(".");

        assert!(matches!(service.decode_jwt(tampered_token), Err(TokenServiceError::JwtError(..))));
    }

    #[test]
    fn test_different_secret() {
        let service = setup_service();
        let fake_service = setup_fake_service();

        // simlate JWT created with different secret
        let fake_jwt = fake_service.create_jwt("test".to_string()).unwrap();

        assert!(matches!(service.decode_jwt(fake_jwt), Err(TokenServiceError::JwtError(..))));
    }

    #[test]
    fn test_unique_iat() {
        let service = setup_service();

        let token1 = service.create_jwt("test".to_string()).unwrap();
        thread::sleep(StdDuration::from_secs(1));
        let token2 = service.create_jwt("test".to_string()).unwrap();

        let claims1 = service.decode_jwt(token1).unwrap();
        let claims2 = service.decode_jwt(token2).unwrap();

        assert!(claims1.iat < claims2.iat);
    }
}
