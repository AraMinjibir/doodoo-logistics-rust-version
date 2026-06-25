use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::domain::{
    errors::domain_error::DomainError,
    models::{user::User, user_status::UserRole},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub iat: usize,
    pub exp: usize,
}
pub struct JwtService {
    secret: String,
    expiry_minutes: i64,
}

impl JwtService {
    pub fn new(secret: String, expiry_minutes: i64) -> Self {
        Self {
            secret,
            expiry_minutes,
        }
    }

    pub fn generate_token(&self, user: &User) -> Result<String, DomainError> {
        let now = Utc::now();
        let expiry = now + Duration::minutes(self.expiry_minutes);

        let claims = JwtClaims {
            sub: user.id().to_string(),
            email: user.email().to_string(),
            name: user.name().to_string(),
            role: user.role().clone(),
            iat: now.timestamp() as usize,
            exp: expiry.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(Into::into)
    }

    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, DomainError> {
        decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(Into::into)
    }
}
