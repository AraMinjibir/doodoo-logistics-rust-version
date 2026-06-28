use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        errors::domain_error::DomainError,
        models::{
            user::{User, UserCommand, UserInput},
            user_status::UserStatus,
        },
        services::{jwt_service::JwtService, user_service::UserService},
    },
    repositories::user_repository::UserRepository,
};

pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepository + Send + Sync>,
    jwt_service: JwtService,
}

impl UserServiceImpl {
    pub fn new(user_repo: Arc<dyn UserRepository + Send + Sync>, jwt_service: JwtService) -> Self {
        Self {
            user_repo,
            jwt_service,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn register_user(&self, user: UserCommand) -> Result<User, DomainError> {
        // Hash the user's password
        let hash_password = User::hash_password_value(user.password());

        // Prevent duplicate for user creation
        let email = user.email();

        if self.user_repo.get_by_email(&email).await?.is_some() {
            return Err(DomainError::UserWithEmailAlreadyExist {
                email: user.email(),
            });
        }

        // Create user
        let new_user = User::create_user(
            user.name(),
            user.email(),
            hash_password?,
            user.phone_number(),
            user.role(),
        )?;

        // Persist user to db
        self.user_repo.create_user(&new_user).await?;

        Ok(new_user)
    }

    async fn login(&self, email: String, password: String) -> Result<String, DomainError> {
        // Fetch user from DB
        let user = self
            .user_repo
            .get_by_email(&email)
            .await?
            .ok_or(DomainError::UserNotFound { email })?;

        // Confirm user's status is Active
        if user.status() != UserStatus::Active {
            return Err(DomainError::UserStatusIsNotActive {
                status: user.status(),
            });
        }
        // Validate user credentials
        if !User::check_password(&password, &user.hash_password())? {
            return Err(DomainError::InvalidCredentials);
        }

        let token = self.jwt_service.generate_token(&user)?;

        Ok(token)
    }
    async fn get_by_id(&self, id: Uuid) -> Result<User, DomainError> {
        let user = self
            .user_repo
            .get_by_id(id)
            .await?
            .ok_or(DomainError::UserNotFoundWithId { id })?;

        Ok(user)
    }
    async fn get_by_email(&self, email: &str) -> Result<User, DomainError> {
        let user = self
            .user_repo
            .get_by_email(email)
            .await?
            .ok_or(DomainError::UserNotFound {
                email: email.to_string(),
            })?;

        Ok(user)
    }

    async fn get_by_status(&self, status: &str) -> Result<Vec<User>, DomainError> {
        let users = self.user_repo.get_by_status(status).await?;
        Ok(users)
    }

    async fn get_by_role(&self, role: &str) -> Result<Vec<User>, DomainError> {
        let users = self.user_repo.get_by_role(role).await?;

        Ok(users)
    }

    async fn get_all(&self) -> Result<Vec<User>, DomainError> {
        let users = self.user_repo.get_all().await?;

        Ok(users)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let affected = self.user_repo.delete(id).await.map_err(DomainError::from)?;

        if affected == 0 {
            Err(DomainError::UserNotFoundWithId { id })
        } else {
            Ok(())
        }
    }

    async fn update_user(&self, id: Uuid, user: UserInput) -> Result<User, DomainError> {
        let existing_user = self
            .user_repo
            .get_by_id(id)
            .await?
            .ok_or(DomainError::UserNotFoundWithId { id })?;
        let updated_user = existing_user.update_user(
            user.name
                .unwrap_or_else(|| existing_user.name().to_string()),
            user.email
                .unwrap_or_else(|| existing_user.email().to_string()),
            user.phone_number
                .unwrap_or_else(|| existing_user.phone_number().to_string()),
            user.role.unwrap_or_else(|| existing_user.role()),
        );

        self.user_repo.update(&updated_user).await?;

        Ok(updated_user)
    }

    async fn update_status(&self, id: Uuid, status: UserStatus) -> Result<User, DomainError> {
        let fetched_user = self
            .user_repo
            .get_by_id(id)
            .await?
            .ok_or(DomainError::UserNotFoundWithId { id })?;

        let updated_user = fetched_user.update_status(status)?;

        self.user_repo.update(&updated_user).await?;

        Ok(updated_user)
    }
}
