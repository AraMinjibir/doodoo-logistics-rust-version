use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::{errors::domain_error::DomainError, models::support_status::SupportStatus};

pub struct Complaint {
    id: Uuid,
    user_id: Uuid,
    shipment_id: Uuid,
    subject: String,
    description: String,
    status: SupportStatus,
    created_at: DateTime<Utc>,
    resolved_at: Option<DateTime<Utc>>,
    resolved_by: Option<Uuid>,
    comment: Vec<Comment>,
}

pub struct Comment {
    id: Uuid,
    complaint_id: Uuid,
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
            resolved_by: None,
            comment: vec![],
        })
    }
}

impl Comment {
    pub fn make_comment(
        complaint_id: Uuid,
        author_id: Uuid,
        message: String,
    ) -> Result<Self, DomainError> {
        let mut errors = Vec::new();

        if complaint_id.is_nil() {
            errors.push("Complaint id must be provided".to_string())
        }
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
            complaint_id,
            author_id,
            message,
            created_at: now,
        })
    }
}
