use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{errors::domain_error::DomainError, models::support_status::SupportStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Complaint {
    id: Uuid,
    user_id: Uuid,
    shipment_id: Uuid,
    subject: String,
    description: String,
    status: SupportStatus,
    created_at: DateTime<Utc>,
    resolved_at: Option<DateTime<Utc>>,
    comment: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    id: Uuid,
    author_id: Uuid,
    message: String,
    created_at: DateTime<Utc>,
}

impl Complaint {
    pub fn send_complaint(
        user_id: Uuid,
        shipment_id: Uuid,
        subject: String,
        description: String,
    ) -> Result<Self, DomainError> {
        let mut errors = Vec::new();

        if user_id.is_nil() {
            errors.push("User id must be provided".to_string());
        }

        if shipment_id.is_nil() {
            errors.push("Shipment id must to be provided".to_string());
        }

        if subject.trim().is_empty() {
            errors.push("Subject cannot be empty".to_string());
        }
        if description.trim().is_empty() {
            errors.push("Complain Description must be provided".to_string());
        }
        if !errors.is_empty() {
            return Err(DomainError::ValidationError(errors));
        }
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            shipment_id,
            subject,
            description,
            status: SupportStatus::Open,
            created_at: now,
            resolved_at: None,
            comment: vec![],
        })
    }
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: Uuid,
        user_id: Uuid,
        shipment_id: Uuid,
        subject: String,
        description: String,
        status: SupportStatus,
        created_at: DateTime<Utc>,
        resolved_at: Option<DateTime<Utc>>,
        comment: Vec<Comment>,
    ) -> Self {
        Self {
            id,
            user_id,
            shipment_id,
            subject,
            description,
            status,
            created_at,
            resolved_at,
            comment,
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
    pub fn shipment_id(&self) -> Uuid {
        self.shipment_id
    }

    pub fn subject(&self) -> String {
        self.subject.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn status(&self) -> SupportStatus {
        self.status.clone()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn resolved_at(&self) -> Option<DateTime<Utc>> {
        self.resolved_at
    }
    pub fn comment(&self) -> Vec<Comment> {
        self.comment.clone()
    }

    pub fn update_status(&self, next: &SupportStatus) -> Result<Self, DomainError> {
        let now = Utc::now();

        SupportStatus::validate_transition(&self.status, next)?;

        Ok(Self {
            status: next.clone(),
            resolved_at: Some(now),
            ..self.clone()
        })
    }
}

impl Comment {
    pub fn make_comment(
        author_id: Uuid,
        message: String,
    ) -> Result<Self, DomainError> {
        let mut errors = Vec::new();

        if author_id.is_nil() {
            errors.push("Author id must be provided".to_string());
        }
        if message.trim().is_empty() {
            errors.push("Message cannot be empty".to_string())
        }

        if !errors.is_empty() {
            return Err(DomainError::ValidationError(errors));
        }

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            author_id,
            message,
            created_at: now,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn author_id(&self) -> Uuid {
        self.author_id
    }


    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
