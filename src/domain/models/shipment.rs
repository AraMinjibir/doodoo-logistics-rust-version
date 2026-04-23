
use uuid::uuid;
use chrono::{DateTime, Utc, Duration};

use crate::domain::models::{
    recipient::Recipient,
    package_details::PackageDetails,
    dimensions::Dimensions,
    address::Address,
    proof_of_delivery::ProofOfDelivery,
    shipment_status::ShipmentStatus
};

#[derive(Debug, Clone)]
pub struct Shipment{
    sender_name: String,
    recipient: Recipient,
    package_details: PackageDetails,
    id: uuid,
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