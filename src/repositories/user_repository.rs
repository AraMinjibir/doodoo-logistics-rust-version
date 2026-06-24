use uuid::Uuid;

use crate::domain::{errors::repository_error::RepositoryError, models::user::User};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<(), RepositoryError>;
    async fn update(&self, user: &User) -> Result<(), RepositoryError>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError>;
    async fn get_by_email(&self, email: String) -> Result<Option<User>, RepositoryError>;
    
    async fn get_by_status(&self, status: &str) -> Result<Vec<User>, RepositoryError>;
    async fn get_by_role(&self, role: &str) -> Result<Vec<User>, RepositoryError>;
    async fn get_all(&self) -> Result<Vec<User>, RepositoryError>;

    async fn delete(&self, id: Uuid) -> Result<u64, RepositoryError>;
}
