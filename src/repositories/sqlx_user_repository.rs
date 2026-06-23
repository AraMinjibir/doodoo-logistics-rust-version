use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::repository_error::map_sqlx_error;
use crate::domain::models::user::User;
use crate::{
    domain::errors::repository_error::RepositoryError, infrastructure::user_row::UserRow,
    repositories::user_repository::UserRepository,
};

pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]

impl UserRepository for SqlxUserRepository {
    async fn create_user(&self, user: &User) -> Result<(), RepositoryError> {
        let row = UserRow::from_domain(user.clone());
        sqlx::query!(
            r#" INSERT INTO users(
            id, name, email, 
            hash_password, phone_number,
            role,status,created_at,updated_at
            
        ) VALUES(
         $1,$2,$3,$4,$5,$6,$7,$8,$9
         )"#,
            row.id,
            row.name,
            row.email,
            row.hash_password,
            row.phone_number,
            row.role,
            row.status,
            row.created_at,
            row.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), RepositoryError> {
        let row = UserRow::from_domain(user.clone());
        sqlx::query!(
            r#"UPDATE users SET
                name = $1,
                email = $2,
                hash_password = $3,
                phone_number = $4,
                role = $5,
                status = $6,
                created_at = $7,
                updated_at = $8
            WHERE id = $9"#,
            row.name,
            row.email,
            row.hash_password,
            row.phone_number,
            row.role,
            row.status,
            row.created_at,
            row.updated_at,
            row.id
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error);

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query_as!(UserRow, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(row.map(UserRow::from_row))
    }
    async fn get_by_status(&self, status: &str) -> Result<Vec<User>, RepositoryError> {
        let rows = sqlx::query_as!(UserRow, "SELECT * FROM users WHERE status = $1", status)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        let users = rows.into_iter().map(UserRow::from_row).collect();

        Ok(users)
    }
    async fn get_by_role(&self, role: &str) -> Result<Vec<User>, RepositoryError> {
        let rows = sqlx::query_as!(UserRow, "SELECT * FROM users WHERE role = $1", role)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;
        let users_with_role = rows.into_iter().map(UserRow::from_row).collect();
        Ok(users_with_role)
    }
    async fn get_all(&self) -> Result<Vec<User>, RepositoryError> {
        let rows = sqlx::query_as!(UserRow, "SELECT * FROM users ORDER BY created_at")
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        let users = rows.into_iter().map(UserRow::from_row).collect();

        Ok(users)
    }

    async fn delete(&self, id: Uuid) -> Result<u64, RepositoryError> {
        let result = sqlx::query!(
            r#"
            UPDATE users
            SET status = $1
            WHERE id = $2
            "#,
            "Deleted",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(result.rows_affected())
    }
}
