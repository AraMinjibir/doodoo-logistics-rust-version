use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::models::{
    user::User,
    user_status::{UserRole, UserStatus},
};

#[derive(Debug, Serialize)]
pub struct SignUpResponse {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    status: UserStatus,
    role: UserRole,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

impl SignUpResponse {
    pub fn from_domain(user: User) -> Self {
        Self {
            id: user.id(),
            name: user.name(),
            email: user.email(),
            password: user.password(),
            status: user.status(),
            role: user.role(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}
