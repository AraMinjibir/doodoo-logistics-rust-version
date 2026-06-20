use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::controllers::dto::shipment_creation_dto::{AddressDto, PackageDetailsDto, RecipientDto};
use crate::domain::models::{proof_of_delivery::ProofOfDelivery, shipment::Shipment};

#[derive(Debug, Serialize)]
pub struct ShipmentResponseDto {
    pub id: uuid::Uuid,
    pub tracking_number: String,

    pub sender_name: String,

    pub recipient: RecipientDto,
    pub package_details: PackageDetailsDto,

    pub cost: String,
    pub status: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub estimated_delivery_date: DateTime<Utc>,

    pub proof_of_delivery: Vec<ProofOfDeliveryResponseDto>,
}

impl From<Shipment> for ShipmentResponseDto {
    fn from(shipment: Shipment) -> Self {
        Self {
            id: shipment.id(),
            tracking_number: shipment.tracking_number().to_string(),

            sender_name: shipment.sender_name().to_string(),

            recipient: RecipientDto {
                name: shipment.recipient().name().to_string(),
                contact: shipment.recipient().contact().to_string(),
                address: AddressDto {
                    country: shipment.recipient().address().country().to_string(),
                    city: shipment.recipient().address().city().to_string(),
                    state: shipment.recipient().address().state().to_string(),
                },
            },

            package_details: PackageDetailsDto {
                dimensions: shipment.package_details().dimensions().clone().into(),
                weight: shipment.package_details().weight().to_string(),
                contents: shipment.package_details().contents().to_string(),
            },

            cost: shipment.cost().to_string(),
            status: shipment.status().to_string(),

            created_at: shipment.created_at(),
            updated_at: shipment.updated_at(),
            estimated_delivery_date: shipment.delivery_date_estimate(),

            proof_of_delivery: shipment
                .proof_of_delivery()
                .iter()
                .cloned()
                .map(ProofOfDeliveryResponseDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProofOfDeliveryResponseDto {
    pub image: Option<String>,
    pub note: String,
    pub submitted_by: String,
}

impl From<ProofOfDelivery> for ProofOfDeliveryResponseDto {
    fn from(p: ProofOfDelivery) -> Self {
        Self {
            image: p.image(),
            note: p.note().to_string(),
            submitted_by: p.submitted_by().to_string(),
        }
    }
}
