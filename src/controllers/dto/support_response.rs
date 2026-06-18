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
    resolved_by: Option<Uuid>,
    comment: Vec<Comment>,
}
impl ComplaintResponse {
    pub fn complaint_response(complaint: Complaint) -> Self {
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
