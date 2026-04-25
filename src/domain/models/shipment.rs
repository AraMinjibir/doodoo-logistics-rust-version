
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

use crate::domain::models::{
    recipient::Recipient,
    package_details::PackageDetails,
    proof_of_delivery::ProofOfDelivery,
    shipment_status::ShipmentStatus
};
use crate::infrastructure::shipment_row::ShipmentRow;

#[derive(Debug, Clone)]
pub struct Shipment{
    sender_name: String,
    recipient: Recipient,
    package_details: PackageDetails,
    id: Uuid,
    tracking_number:  Option<String>,
    status: ShipmentStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    proof_of_delivery: Vec<ProofOfDelivery>,
    service_provider_id: Option<Uuid>,
}

impl Shipment {
    pub fn new(
        sender_name: String,
        recipient: Recipient,
        package_details: PackageDetails,
        service_provider_id: Option<Uuid>,
    ) -> Self {
        Self {
            sender_name,
            recipient,
            package_details,
            id: Uuid::new_v4(),
            tracking_number: Some(Self::generate_tracking_number()),
            status: ShipmentStatus::Created,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            proof_of_delivery: vec![],
            service_provider_id,
        }
    }
}

impl Shipment {
    fn generate_tracking_number() -> String {
        format!("TRK-{}", Uuid::new_v4())
    }
}

impl Shipment {
    pub fn cost(&self) -> f64 {
        let weight = self.package_details.weight();
        let base_price = 10.0;
        let rate_per_kg = 2.5;

        base_price + (weight * rate_per_kg)
    }
}

impl Shipment {
    pub fn delivery_date_estimate(&self) -> DateTime<Utc> {
        Utc::now() + Duration::days(3)
    }
}

impl From<ShipmentRow> for Shipment {
    fn from(row: ShipmentRow) -> Self {
        Shipment {
            id: row.id,
            tracking_number: row.tracking_number,
            sender_name: row.sender_name,

            recipient: todo!(),
            package_details: todo!(),

            status: ShipmentStatus::from_string(&row.status)
                .expect("Invalid shipment status in DB"),

            created_at: DateTime::<Utc>::from_naive_utc_and_offset(row.created_at, Utc),
            updated_at: DateTime::<Utc>::from_naive_utc_and_offset(row.updated_at, Utc),

            proof_of_delivery: vec![],
            service_provider_id: row.service_provider_id,
        }
    }
}

impl From<Shipment> for ShipmentRow {
    fn from(shipment: Shipment) -> Self {
        ShipmentRow {
            id: shipment.id,
            tracking_number: shipment.tracking_number.clone(),
            sender_name: shipment.sender_name.clone(),
            recipient_name: shipment.recipient.name().to_string(),
            recipient_street: shipment.recipient.address().street().to_string(),
            recipient_city: shipment.recipient.address().city().to_string(),
            recipient_state: shipment.recipient.address().state().to_string(),
            recipient_country: shipment.recipient.address().country().to_string(),
            recipient_postal_code: shipment.recipient.address().postal_code().to_string(),
            recipient_contact: shipment.recipient.contact().to_string(),

            weight: shipment.package_details.weight(),
            width: shipment.package_details.dimensions().width(),
            length: shipment.package_details.dimensions().length(),
            height: shipment.package_details.dimensions().height(),
            contents: shipment.package_details.contents().to_string(),
            status: shipment.status.to_string(),
            estimated_delivery_date: None,
            created_at: shipment.created_at.naive_local(),
            updated_at: shipment.updated_at.naive_local(),
            cost: shipment.cost(),
            proof_of_delivery: serde_json::json!(shipment.proof_of_delivery),
            service_provider_id: shipment.service_provider_id,
        }
    }
}