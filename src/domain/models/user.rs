use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use uuid::Uuid;

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
#[derive(Debug, Clone)]
pub struct UserInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<UserRole>,
}

#[derive(Debug, Clone)]
pub struct UserCommand {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub role: UserRole,
}
impl UserCommand {
    pub fn new(
        name: String,
        email: String,
        password: String,
        phone_number: String,
        role: UserRole,
    ) -> Self {
        Self {
            name,
            email,
            password,
            phone_number,
            role,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn password(&self) -> String {
        self.password.clone()
    }
    pub fn phone_number(&self) -> String {
        self.phone_number.clone()
    }
    pub fn role(&self) -> UserRole {
        self.role.clone()
    }
}
#[allow(dead_code)]
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

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: Uuid,
        name: String,
        email: String,
        hash_password: String,
        phone_number: String,
        role: UserRole,
        status: UserStatus,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            name,
            email,
            hash_password,
            phone_number,
            role,
            status,
            created_at,
            updated_at,
        }
    }
    pub fn update_user(
        &self,
        name: String,
        email: String,
        phone_number: String,
        role: UserRole,
    ) -> Self {
        Self {
            name,
            email,
            phone_number,
            role,
            updated_at: Some(Utc::now()),
            ..self.clone()
        }
    }
    pub fn update_status(&self, next: UserStatus) -> Result<Self, DomainError> {
        let now = Utc::now();

        Ok(Self {
            status: next,
            updated_at: Some(now),
            ..self.clone()
        })
    }
    pub fn hash_password_value(plain_password: String) -> Result<String, bcrypt::BcryptError> {
        hash(plain_password, DEFAULT_COST)
    }

    pub fn check_password(
        plain_password: &str,
        hash_password: &str,
    ) -> Result<bool, bcrypt::BcryptError> {
        verify(plain_password, hash_password)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn hash_password(&self) -> String {
        self.hash_password.clone()
    }
    pub fn password(&self) -> String {
        self.hash_password.clone()
    }
    pub fn phone_number(&self) -> String {
        self.phone_number.clone()
    }
    pub fn role(&self) -> UserRole {
        self.role.clone()
    }
    pub fn status(&self) -> UserStatus {
        self.status.clone()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }

    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
        self.updated_at = Some(Utc::now());
    }
}
