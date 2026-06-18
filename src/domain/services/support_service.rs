use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::domain_error::DomainError;
use crate::domain::models::support::Comment;
use crate::domain::models::{support::Complaint, support_status::SupportStatus};

#[async_trait]
pub trait SupportService {
    async fn send_complaint(&self, complaint: &Complaint) -> Result<Complaint, DomainError>;
    async fn send_comment(
        &self,
        complaint_id: Uuid,
        comment: Comment,
    ) -> Result<Complaint, DomainError>;

    async fn get_complaint_by_id(&self, id: Uuid) -> Result<Complaint, DomainError>;
    async fn get_complaint_by_status(
        &self,
        status: &SupportStatus,
    ) -> Result<Vec<Complaint>, DomainError>;

    async fn get_all_compalints(&self) -> Result<Vec<Complaint>, DomainError>;

    async fn update_complaint_status(
        &self,
        complaint_id: Uuid,
        status: &SupportStatus,
        complaint: &Complaint,
    ) -> Result<Complaint, DomainError>;
    async fn delete_complaint(&self, id: Uuid) -> Result<(), DomainError>;
}
