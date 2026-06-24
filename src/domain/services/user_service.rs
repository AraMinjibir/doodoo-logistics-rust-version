use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    errors::domain_error::DomainError,
    models::{
        user::{User, UserInput},
        user_status::UserStatus,
    },
};

#[async_trait]
pub trait UserService {
    async fn register_user(&self, user: &User) -> Result<User, DomainError>;
    async fn login(&self, email: String, password: String) -> Result<String, DomainError>;
    async fn update_user(&self, id: Uuid, user: UserInput) -> Result<User, DomainError>;
    async fn update_status(&self, id: Uuid, status: UserStatus) -> Result<User, DomainError>;

    async fn get_by_id(&self, id: Uuid) -> Result<User, DomainError>;
    async fn get_by_status(&self, status: &str) -> Result<Vec<User>, DomainError>;
    async fn get_by_role(&self, role: &str) -> Result<Vec<User>, DomainError>;

    async fn get_all(&self) -> Result<Vec<User>, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
