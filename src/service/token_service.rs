use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct TokenService {
    decoding_key: DecodingKey,
    encoding_key: EncodingKey,
}

impl TokenService {
    pub fn new(secret: String) -> Self {
        let secret = secret.as_bytes();

        Self { 
            decoding_key: DecodingKey::from_secret(secret),
            encoding_key: EncodingKey::from_secret(secret),
        }
    }

    pub fn create_jwt(&self, user_id: String) -> String {
        let iat = Utc::now().timestamp() as usize;

        let exp = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            iat,
            exp,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .expect("Token creation failed")
    }

    pub fn decode_jwt(&self, token: String) -> Claims {
        let validation = Validation::default();
        let token = decode::<Claims>(&token, &self.decoding_key, &validation)
            .expect("Token decoding failed");

        token.claims
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
        let user_id = "user123".to_string();

        let token = service.create_jwt(user_id.clone());
        let claims = service.decode_jwt(token);

        assert_eq!(claims.iss, SERVICE_NAME.to_string());
        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_claims_content() {
        let service = setup_service();
        let user_id = "alice".to_string();

        let token = service.create_jwt(user_id.clone());
        let claims = service.decode_jwt(token);

        assert_eq!(claims.sub, "alice");
        let now = Utc::now().timestamp() as usize;
        assert!(claims.iat <= now);
        assert!(claims.exp > now);
    }

    #[test]
    #[should_panic]
    fn test_invalid_token() {
        let service = setup_service();

        let invalid_token = "this.is.not.valid".to_string();

        service.decode_jwt(invalid_token);
    }

    #[test]
    #[should_panic]
    fn test_tampered_token() {
        let service = setup_service();
        let token = service.create_jwt("bob".to_string());

        let mut parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);

        parts[1] = "tampered_payload";
        let tampered_token = parts.join(".");

        service.decode_jwt(tampered_token);
    }

    #[test]
    #[should_panic]
    fn test_different_secret() {
        let service = setup_service();
        let fake_service = setup_fake_service();

        // simlate JWT created with different secret
        let fake_jwt = fake_service.create_jwt("test".to_string());

        service.decode_jwt(fake_jwt);
    }

    #[test]
    fn test_unique_iat() {
        let service = setup_service();

        let token1 = service.create_jwt("test".to_string());
        thread::sleep(StdDuration::from_secs(1));
        let token2 = service.create_jwt("test".to_string());

        let claims1 = service.decode_jwt(token1);
        let claims2 = service.decode_jwt(token2);

        assert!(claims1.iat < claims2.iat);
    }
}
