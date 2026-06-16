use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::repository_error::map_sqlx_error;
use crate::domain::models::support::Complaint;
use crate::{
    domain::errors::repository_error::RepositoryError, infrastructure::support_row::ComplaintRow,
    repositories::support_repository::SupportRepository,
};

pub struct SqlxSupportRepository {
    pool: PgPool,
}

impl SqlxSupportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SupportRepository for SqlxSupportRepository {
    async fn persist_complaint(&self, complaint: &Complaint) -> Result<(), RepositoryError> {
        let row = ComplaintRow::from_complaint_domain(complaint);
        sqlx::query!( r#"
        INSERT INTO support
        (
        id, user_id, shipment_id, subject, description, status,created_at, resolved_at, resolved_by, comment
        ) VALUES ($1,$2, $3, $4, $5, $6, $7, $8, $9, $10)
         "#,
         row.id,
         row.user_id,
         row.shipment_id,
         row.subject,
         row.description,
         row.status.to_string(),
         row.created_at,
         row.resolved_at,
         row.resolved_by,
         row.comment,
        ).execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(())
    }
    async fn persist_comment(
        &self,
        complaint_id: Uuid,
        comment: serde_json::Value,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
               UPDATE support
               SET comment = $1,
                   created_at = NOW()
                WHERE id = $2
                "#,
            comment,
            complaint_id
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn get_complaint_by_id(&self, id: Uuid) -> Result<Option<Complaint>, RepositoryError> {
        let row = sqlx::query_as!(ComplaintRow, "SELECT * FROM support WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(row.map(ComplaintRow::into_complaint_domain))
    }

    async fn get_complaint_by_status(
        &self,
        status: &str,
    ) -> Result<Vec<Complaint>, RepositoryError> {
        let row = sqlx::query_as!(
            ComplaintRow,
            "SELECT * FROM support WHERE status = $1",
            status
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        let complaints = row
            .into_iter()
            .map(ComplaintRow::into_complaint_domain)
            .collect();

        Ok(complaints)
    }

    async fn get_all_compalints(&self) -> Result<Vec<Complaint>, RepositoryError> {
        let row = sqlx::query_as!(ComplaintRow, "SELECT * FROM support ORDER BY created_at")
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        let complaints = row
            .into_iter()
            .map(ComplaintRow::into_complaint_domain)
            .collect();

        Ok(complaints)
    }

    async fn update_complaint_status(
        &self,
        status: &str,
        complaint: &Complaint,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            UPDATE support SET
            status = $1,
            resolved_at = NOW()
            WHERE id = $2
            "#,
            status,
            complaint.id()
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn delete_complaint(&self, id: Uuid) -> Result<u64, RepositoryError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM support 
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(result.rows_affected())
    }
}
