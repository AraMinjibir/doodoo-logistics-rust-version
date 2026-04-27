use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

use crate::domain::models::{
    recipient::Recipient,
    package_details::PackageDetails,
    proof_of_delivery::ProofOfDelivery,
    shipment_status::ShipmentStatus,
};

#[derive(Debug, Clone)]
pub struct Shipment {
    id: Uuid,
    sender_name: String,
    recipient: Recipient,
    package_details: PackageDetails,
    tracking_number: String,
    status: ShipmentStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    proof_of_delivery: Vec<ProofOfDelivery>,
    service_provider_id: Option<Uuid>,
}

impl Shipment {
    pub fn create(
        sender_name: String,
        recipient: Recipient,
        package_details: PackageDetails,
        service_provider_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            sender_name,
            recipient,
            package_details,
            tracking_number: Self::generate_tracking_number(),
            status: ShipmentStatus::Created,
            created_at: now,
            updated_at: now,
            proof_of_delivery: vec![],
            service_provider_id,
        }
    }

    pub fn reconstitute(
        id: Uuid,
        sender_name: String,
        recipient: Recipient,
        package_details: PackageDetails,
        tracking_number: String,
        status: ShipmentStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        proof_of_delivery: Vec<ProofOfDelivery>,
        service_provider_id: Option<Uuid>,
    ) -> Self {
        Self {
            id,
            sender_name,
            recipient,
            package_details,
            tracking_number,
            status,
            created_at,
            updated_at,
            proof_of_delivery,
            service_provider_id,
        }
    }

    // Getters

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn sender_name(&self) -> &str {
        &self.sender_name
    }

    pub fn status(&self) -> ShipmentStatus {
        self.status.clone()
    }

    pub fn service_provider_id(&self) -> Option<Uuid> {
        self.service_provider_id
    }

    pub fn set_service_provider_id(&mut self, id: Option<Uuid>) {
        self.service_provider_id = id;
        self.updated_at = Utc::now();
    }

    pub fn set_sender_name(&mut self, name: String) {
        self.sender_name = name;
        self.updated_at = Utc::now();
    }

    pub fn set_status(&mut self, status: ShipmentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn proof_of_delivery(&self) -> &Vec<ProofOfDelivery> {
        &self.proof_of_delivery
    }

    pub fn recipient(&self) -> &Recipient {
        &self.recipient
    }

    pub fn package_details(&self) -> &PackageDetails{
        &self.package_details
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
    
    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }

    // Business logic

    pub fn cost(&self) -> f64 {
        let base = 10.0;
        let rate = 2.5;
        base + (self.package_details.weight() * rate)
    }

    pub fn delivery_date_estimate(&self) -> DateTime<Utc> {
        Utc::now() + Duration::days(3)
    }

    fn generate_tracking_number() -> String {
        format!("TRK-{}", Uuid::new_v4())
    }

    pub fn tracking_number(&self) -> &str {
        &self.tracking_number
    }
}