use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ShipmentRow {
    pub id: Uuid,
    pub tracking_number: Option<String>,
    pub sender_name: String,

    pub recipient_name: String,
    pub recipient_street: String,
    pub recipient_city: String,
    pub recipient_state: String,
    pub recipient_country: String,
    pub recipient_postal_code: String,
    pub recipient_contact: String,

    pub weight: f64,
    pub width: f64,
    pub length: f64,
    pub height: f64,

    pub contents: String,
    pub status: String,

    pub estimated_delivery_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub cost: f64,

    pub proof_of_delivery: serde_json::Value,
    pub service_provider_id: Option<Uuid>,
}
