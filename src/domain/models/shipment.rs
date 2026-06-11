use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::domain::errors::domain_error::DomainError;
use crate::domain::models::{
    package_details::PackageDetails, proof_of_delivery::ProofOfDelivery, recipient::Recipient,
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

#[allow(dead_code)]
impl Shipment {
    pub fn create(
        sender_name: String,
        recipient: Recipient,
        package_details: PackageDetails,
        service_provider_id: Option<Uuid>,
    ) -> Result<Self, DomainError> {
        let mut errors = Vec::new();

        if sender_name.trim().is_empty() {
            errors.push("Sender name must not be empty".to_string());
        }

        if !errors.is_empty() {
            return Err(DomainError::ValidationError(errors));
        }

        let now = Utc::now();
        Ok(Self {
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
        })
    }
    #[allow(clippy::too_many_arguments)]
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

    pub fn updated_shipment(
        &self,
        sender_name: String,
        recipient: Recipient,
        package_details: PackageDetails,
    ) -> Self {
        Self {
            sender_name,
            recipient,
            package_details,
            updated_at: Utc::now(),
            ..self.clone()
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

    pub fn proof_of_delivery(&self) -> &[ProofOfDelivery] {
        &self.proof_of_delivery
    }

    pub fn recipient(&self) -> &Recipient {
        &self.recipient
    }

    pub fn package_details(&self) -> &PackageDetails {
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

    pub fn update_status(&self, next: ShipmentStatus) -> Result<Self, DomainError> {
        let now = Utc::now();

        ShipmentStatus::validate_transition(&self.status, &next)?;

        Ok(Self {
            status: next,
            updated_at: now,
            ..self.clone()
        })
    }

    pub fn attach_proof_of_delivery(&self, proof: ProofOfDelivery) -> Result<Self, DomainError> {
        // 1. Must be delivered
        if self.status != ShipmentStatus::Delivered {
            return Err(DomainError::ShipmentNotDelivered);
        }

        // 2. Prevent duplicate submitter
        if self
            .proof_of_delivery
            .iter()
            .any(|p| p.submitted_by() == proof.submitted_by())
        {
            return Err(DomainError::DuplicateProofOfDelivery);
        }

        // 3. Return updated immutable copy
        Ok(Self {
            proof_of_delivery: {
                let mut proofs = self.proof_of_delivery.clone();
                proofs.push(proof);
                proofs
            },
            updated_at: Utc::now(),
            ..self.clone()
        })
    }
}

pub struct UpdateShipment {
    pub sender_name: Option<String>,
    pub recipient: Option<Recipient>,
    pub package_details: Option<PackageDetails>,
}
