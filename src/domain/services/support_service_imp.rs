use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::domain_error::DomainError;
use crate::domain::models::support::Comment;
use crate::domain::models::{support::Complaint, support_status::SupportStatus};
use crate::{
    domain::services::support_service::SupportService,
    repositories::support_repository::SupportRepository,
};

pub struct SupportServiceImpl {
    repo: Arc<dyn SupportRepository + Send + Sync>,
}

impl SupportServiceImpl {
    pub fn new(repo: Arc<dyn SupportRepository + Send + Sync>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl SupportService for SupportServiceImpl {
    async fn send_complaint(&self, complaint: &Complaint) -> Result<Complaint, DomainError> {
        self.repo.persist_complaint(&complaint).await?;
        Ok(complaint.clone())
    }

    async fn send_comment(
        &self,
        complaint_id: Uuid,
        comment: Comment,
    ) -> Result<Complaint, DomainError> {
        self.repo
            .get_complaint_by_id(complaint_id)
            .await
            .map_err(DomainError::from)?
            .ok_or(DomainError::ComplaintNotFound { id: complaint_id })?;

        self.repo
            .persist_comment(
                complaint_id,
                serde_json::to_value(&comment)
                    .map_err(|e| DomainError::ValidationError(vec![e.to_string()]))?,
            )
            .await
            .map_err(DomainError::from)?;

        let updated_complaint = self
            .repo
            .get_complaint_by_id(complaint_id)
            .await
            .map_err(DomainError::from)?
            .ok_or(DomainError::ComplaintNotFound { id: complaint_id })?;

        Ok(updated_complaint)
    }

    async fn get_complaint_by_id(&self, id: Uuid) -> Result<Complaint, DomainError> {
        let fetched_complaint = self.repo.get_complaint_by_id(id).await?;

        match fetched_complaint {
            Some(complaint) => Ok(complaint),
            None => Err(DomainError::ComplaintNotFound { id }),
        }
    }

    async fn get_complaint_by_status(
        &self,
        status: &SupportStatus,
    ) -> Result<Vec<Complaint>, DomainError> {
        self.repo
            .get_complaint_by_status(status)
            .await
            .map_err(DomainError::from)
    }

    async fn get_all_compalints(&self) -> Result<Vec<Complaint>, DomainError> {
        self.repo
            .get_all_compalints()
            .await
            .map_err(DomainError::from)
    }

    async fn update_complaint_status(
        &self,
        complaint_id: Uuid,
        status: &SupportStatus,
    ) -> Result<Complaint, DomainError> {
        // Find complaint first
        let complaint = self
            .repo
            .get_complaint_by_id(complaint_id)
            .await
            .map_err(DomainError::from)?
            .ok_or(DomainError::ComplaintNotFound { id: complaint_id })?;

        let updated_complaint = complaint.update_status(status)?;

        self.repo
            .update_complaint_status(&status, &updated_complaint)
            .await
            .map_err(DomainError::from)?;

        Ok(updated_complaint)
    }

    async fn delete_complaint(&self, id: Uuid) -> Result<(), DomainError> {
        let affected_row = self
            .repo
            .delete_complaint(id)
            .await
            .map_err(DomainError::from)?;

        if affected_row == 0 {
            Err(DomainError::ComplaintNotFound { id })
        } else {
            Ok(())
        }
    }
}
