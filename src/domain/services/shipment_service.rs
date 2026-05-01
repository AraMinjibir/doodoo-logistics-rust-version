use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::{
    shipment::Shipment,
    shipment::UpdateShipment
};
use crate::domain::models::shipment_status::ShipmentStatus;
use crate::domain::models::proof_of_delivery::ProofOfDelivery;
use crate::domain::errors::domain_error::DomainError;

#[async_trait]
pub trait ShipmentService {

    async fn create_shipment(
        &self,
        shipment: Shipment,
    ) -> Result<Shipment, DomainError>;

    async fn get_by_tracking_number(
        &self,
        tracking: &str,
    ) -> Result<Shipment, DomainError>;

    async fn get_by_id(
        &self,
        id: Uuid,
    ) -> Result<Shipment, DomainError>;

    async fn get_by_status(
        &self,
        status: ShipmentStatus,
    ) -> Result<Vec<Shipment>, DomainError>;

    async fn update_status(
        &self,
        tracking: &str,
        status: ShipmentStatus,
        location: Option<String>,
    ) -> Result<Shipment, DomainError>;

    async fn update_shipment(
        &self,
        id: Uuid,
        dto: UpdateShipment,
    ) -> Result<Shipment, DomainError>;

    async fn list_shipments(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Shipment>, DomainError>;

    async fn delete_shipment(
        &self,
        id: Uuid,
    ) -> Result<(), DomainError>;

    async fn upload_proof_of_delivery(
        &self,
        tracking: &str,
        proof: ProofOfDelivery,
    ) -> Result<Shipment,DomainError>;

    
}