use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::models::support::{Comment, Complaint};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ComplaintRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub shipment_id: Uuid,
    pub subject: String,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub comment: serde_json::Value,
}

impl ComplaintRow {
    fn deserialize_pod(value: serde_json::Value) -> Vec<Comment> {
        serde_json::from_value(value).unwrap_or_else(|_| vec![])
    }
    pub fn into_complaint_domain(self) -> Complaint {
        Complaint::reconstitute(
            self.id,
            self.user_id,
            self.shipment_id,
            self.subject,
            self.description,
            self.status.parse().expect("Invalid complaint status in DB"),
            self.created_at,
            self.resolved_at,
            Self::deserialize_pod(self.comment),
        )
    }

    pub fn from_complaint_domain(complaint: &Complaint) -> Self {
        Self {
            id: complaint.id(),
            user_id: complaint.user_id(),
            shipment_id: complaint.shipment_id(),
            subject: complaint.subject(),
            description: complaint.description(),
            status: complaint.status().to_string(),
            created_at: complaint.created_at(),
            resolved_at: complaint.resolved_at(),
            comment: serde_json::to_value(complaint.comment()).unwrap(),
        }
    }
}
