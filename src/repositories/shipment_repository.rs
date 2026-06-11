use crate::domain::errors::repository_error::RepositoryError;
use crate::domain::models::shipment::Shipment;
use std::result::Result;
use uuid::Uuid;

#[async_trait::async_trait]
#[allow(dead_code)]
pub trait ShipmentRepository: Send + Sync {
    async fn create(&self, shipment: &Shipment) -> Result<(), RepositoryError>;
    async fn update(&self, shipment: &Shipment) -> Result<(), RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<u64, RepositoryError>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Shipment>, RepositoryError>;

    async fn get_by_status(&self, status: &str) -> Result<Vec<Shipment>, RepositoryError>;

    async fn find_by_tracking_number(
        &self,
        tracking: &str,
    ) -> Result<Option<Shipment>, RepositoryError>;

    async fn list_all(&self, offset: i64, limit: i64) -> Result<Vec<Shipment>, RepositoryError>;

    async fn assign_service_provider(
        &self,
        shipment_id: Uuid,
        provider_id: Uuid,
    ) -> Result<(), RepositoryError>;

    async fn upload_proof_of_delivery(
        &self,
        shipment_id: Uuid,
        proof: serde_json::Value,
    ) -> Result<Option<Shipment>, RepositoryError>;

    async fn find_by_service_provider(
        &self,
        provider_id: Uuid,
    ) -> Result<Vec<Shipment>, RepositoryError>;
}
