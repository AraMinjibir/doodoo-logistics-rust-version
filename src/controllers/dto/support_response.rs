use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::{
    support::{Comment, Complaint},
    support_status::SupportStatus,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ComplaintResponse {
    id: Uuid,
    user_id: Uuid,
    shipment_id: Uuid,
    subject: String,
    description: String,
    status: SupportStatus,
    created_at: DateTime<Utc>,
    resolved_at: Option<DateTime<Utc>>,
    comment: Vec<CommentResponseDto>,
}
impl ComplaintResponse {
    pub fn new(complaint: Complaint) -> Self {
        Self {
            id: complaint.id(),
            user_id: complaint.user_id(),
            shipment_id: complaint.shipment_id(),
            subject: complaint.subject(),
            description: complaint.description(),
            status: complaint.status(),
            created_at: complaint.created_at(),
            resolved_at: complaint.resolved_at(),
            comment: complaint.comment().iter().cloned().map(CommentResponseDto::from).collect(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentResponseDto {
    id: Uuid,
    author_id: Uuid,
    message: String,
    created_at: DateTime<Utc>,
}

impl From<Comment> for CommentResponseDto {

    fn from(comment:Comment) -> Self {
        Self { id: comment.id(), author_id: comment.author_id(),
             message: comment.message(), 
             created_at: comment.created_at() 
            }
    }
    
}