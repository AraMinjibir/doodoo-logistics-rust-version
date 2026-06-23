use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::models::{
    user::User,
    user_status::{UserRole, UserStatus},
};

pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub hash_password: String,
    pub phone_number: String,
    pub role: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl UserRow {
    // Converting from DB Row into Domain Model
    pub fn from_row(self) -> User {
        User::reconstitute(
            self.id,
            self.name,
            self.email,
            self.hash_password,
            self.phone_number,
            UserRole::string_roles(&self.role).expect("Invalid User role from DB."),
            UserStatus::from_string(&self.status).expect("Invalid User status from DB."),
            self.created_at,
            self.updated_at,
        )
    }

    // Converting form Domain Model  into  DB Row

    pub fn from_domain(user: User) -> Self {
        Self {
            id: user.id(),
            name: user.name(),
            email: user.email(),
            hash_password: user.hash_password(),
            phone_number: user.phone_number(),
            role: user.role().to_string(),
            status: user.status().to_string(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}
