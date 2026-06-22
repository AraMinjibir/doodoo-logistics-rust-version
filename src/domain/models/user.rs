use chrono::{DateTime, Utc};
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST, verify};

use crate::domain::{
    errors::domain_error::DomainError,
    models::user_status::{UserRole, UserStatus},
};

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    hash_password: String,
    phone_number: String,
    role: UserRole,
    status: UserStatus,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn create_user(
        name: String,
        email: String,
        password: String,
        phone_number: String,
        role: UserRole,
    ) -> Result<Self, DomainError> {
        let mut errors = Vec::new();

        if name.trim().is_empty() {
            errors.push("Name is required".to_string());
        }
        if email.trim().is_empty() {
            errors.push("Email address is required".to_string());
        }
        if password.trim().is_empty() {
            errors.push("Password must be provided".to_string());
        }

        if password.len() < 8 {
            errors.push("Password must be at least 8 characters".to_string());
        }
        if !errors.is_empty() {
            return Err(DomainError::ValidationError(errors));
        }

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            email,
            hash_password: password,
            phone_number,
            role,
            status: UserStatus::Active,
            created_at: now,
            updated_at: None,
        })
    }

    pub fn hash_password_value(plain_password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(plain_password, DEFAULT_COST)
    }

    pub fn check_password(
        plain_password: &str,
        hash_password: &str,
    ) -> Result<bool, bcrypt::BcryptError> {
        verify(plain_password, hash_password)
    }
}
