use uuid::Uuid;

use crate::domain::{errors::repository_error::RepositoryError, models::{support::Complaint, support_status::SupportStatus}};

#[async_trait::async_trait]
pub trait SupportRepository: Send + Sync {
    async fn persist_complaint(&self, complaint: &Complaint) -> Result<(), RepositoryError>;
    async fn persist_comment(
        &self,
        complaint_id: Uuid,
        comment: serde_json::Value,
    ) -> Result<(), RepositoryError>;

    async fn get_complaint_by_id(&self, id: Uuid) -> Result<Option<Complaint>, RepositoryError>;
    async fn get_complaint_by_status(
        &self,
        status: &SupportStatus,
    ) -> Result<Vec<Complaint>, RepositoryError>;

    async fn get_all_compalints(&self) -> Result<Vec<Complaint>, RepositoryError>;

    async fn update_complaint_status(
        &self,
        status: &SupportStatus,
        complaint: &Complaint,
    ) -> Result<(), RepositoryError>;
    async fn delete_complaint(&self, id: Uuid) -> Result<u64, RepositoryError>;
}
