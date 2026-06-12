use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::models::{
    support::{Comment, Complaint},
    support_status::SupportStatus,
};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ComplaintRow {
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

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CommentRow {
    id: Uuid,
    complaint_id: Uuid,
    author_id: Uuid,
    message: String,
    created_at: DateTime<Utc>,
}

impl ComplaintRow {
    pub fn into_complaint_domain(self) -> Complaint {
        Complaint::reconstitute(
            self.id,
            self.user_id,
            self.shipment_id,
            self.subject,
            self.description,
            self.status,
            self.created_at,
            self.resolved_at,
            self.resolved_by,
            self.comment,
        )
    }

    pub fn from_complaint_domain(complaint: &Complaint) -> Self {
        Self {
            id: complaint.id(),
            user_id: complaint.user_id(),
            shipment_id: complaint.shipment_id(),
            subject: complaint.subject(),
            description: complaint.description(),
            status: complaint.status(),
            created_at: complaint.created_at(),
            resolved_at: complaint.resolved_at(),
            resolved_by: complaint.resolved_by(),
            comment: complaint.comment(),
        }
    }
}

impl CommentRow {
    pub fn into_comment_domain(self) -> Comment {
        Comment::reconstitute_comment(
            self.id,
            self.complaint_id,
            self.author_id,
            self.message,
            self.created_at,
        )
    }

    pub fn from_comment_domain(comment: Comment) -> Self {
        Self {
            id: comment.id(),
            complaint_id: comment.complaint_id(),
            author_id: comment.author_id(),
            message: comment.message(),
            created_at: comment.created_at(),
        }
    }
}
