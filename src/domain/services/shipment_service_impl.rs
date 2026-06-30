use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::domain_error::DomainError;
use crate::domain::models::proof_of_delivery::ProofOfDelivery;
use crate::domain::models::shipment::{Shipment, UpdateShipment};
use crate::domain::models::shipment_status::ShipmentStatus;
use crate::domain::models::user_status::UserRole;
use crate::domain::services::shipment_service::ShipmentService;
use crate::repositories::{
    shipment_repository::ShipmentRepository, user_repository::UserRepository,
};

pub struct ShipmentServiceImpl {
    repo: Arc<dyn ShipmentRepository + Send + Sync>,
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}
impl ShipmentServiceImpl {
    pub fn new(
        repo: Arc<dyn ShipmentRepository + Send + Sync>,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    ) -> Self {
        Self { repo, user_repo }
    }
}

#[async_trait]
impl ShipmentService for ShipmentServiceImpl {
    async fn create_shipment(&self, shipment: Shipment) -> Result<Shipment, DomainError> {
        self.repo.create(&shipment).await?;

        Ok(shipment)
    }
    async fn get_by_tracking_number(&self, tracking: &str) -> Result<Shipment, DomainError> {
        let shipments = self.repo.find_by_tracking_number(tracking).await?.ok_or(
            DomainError::ShipmentNotFound {
                tracking_number: tracking.to_string(),
            },
        )?;

        Ok(shipments)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Shipment, DomainError> {
        let shipment_opt = self.repo.get_by_id(id).await?;

        match shipment_opt {
            Some(shipment) => Ok(shipment),
            None => Err(DomainError::ShipmentNotFoundById { id }),
        }
    }
    async fn get_by_provider_id(&self, provider_id: Uuid) -> Result<Vec<Shipment>, DomainError> {
        let assigned = self.repo.find_by_service_provider(provider_id).await?;

        Ok(assigned)
    }
    async fn list_shipments(&self, offset: i64, limit: i64) -> Result<Vec<Shipment>, DomainError> {
        let shipments = self.repo.list_all(offset, limit).await?;

        Ok(shipments)
    }
    async fn get_by_status(&self, status: ShipmentStatus) -> Result<Vec<Shipment>, DomainError> {
        let shipments = self.repo.get_by_status(&status.to_string()).await?;

        Ok(shipments)
    }

    async fn update_status(
        &self,
        tracking: &str,
        status: ShipmentStatus,
        _location: Option<String>,
    ) -> Result<Shipment, DomainError> {
        let shipment = self
            .repo
            .find_by_tracking_number(tracking)
            .await
            .map_err(DomainError::from)?
            .ok_or(DomainError::ShipmentNotFound {
                tracking_number: tracking.to_string(),
            })?;
        let updated = shipment.update_status(status)?;

        self.repo
            .update(&updated)
            .await
            .map_err(DomainError::from)?;
        Ok(updated)
    }

    async fn delete_shipment(&self, id: Uuid) -> Result<(), DomainError> {
        let affected = self.repo.delete(id).await.map_err(DomainError::from)?;

        if affected == 0 {
            Err(DomainError::ShipmentNotFoundById { id })
        } else {
            Ok(())
        }
    }

    async fn upload_proof_of_delivery(
        &self,
        tracking: &str,
        proof: ProofOfDelivery,
    ) -> Result<Shipment, DomainError> {
        // 1. Validate proof
        let valid_proof =
            ProofOfDelivery::create(proof.image(), proof.note(), proof.submitted_by()).map_err(
                |errs| {
                    DomainError::ValidationError(errs.into_iter().map(|e| e.to_string()).collect())
                },
            )?;

        // 2. Fetch shipment
        let shipment = self.repo.find_by_tracking_number(tracking).await?.ok_or(
            DomainError::ShipmentNotFound {
                tracking_number: tracking.to_string(),
            },
        )?;

        // 3. Domain logic
        let updated = shipment.attach_proof_of_delivery(valid_proof)?;

        // 4. Serialize
        let proof_json = serde_json::to_value(updated.proof_of_delivery())?;

        // 5. Persist
        let saved = self
            .repo
            .upload_proof_of_delivery(updated.id(), proof_json)
            .await?
            .ok_or(DomainError::DuplicateProofOfDelivery)?;

        Ok(saved)
    }

    async fn update_shipment(
        &self,
        id: Uuid,
        input: UpdateShipment,
    ) -> Result<Shipment, DomainError> {
        let existing = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or(DomainError::ShipmentNotFoundById { id })?;

        let updated = existing.updated_shipment(
            input
                .sender_name
                .unwrap_or_else(|| existing.sender_name().to_string()),
            input
                .recipient
                .unwrap_or_else(|| existing.recipient().clone()),
            input
                .package_details
                .unwrap_or_else(|| existing.package_details().clone()),
        );

        self.repo.update(&updated).await?;

        Ok(updated)
    }

    async fn assign_service_provider(
        &self,
        shipment_id: Uuid,
        provider_id: Uuid,
    ) -> Result<Shipment, DomainError> {
        //1 Check whether the shipment exist

        let shipment = self
            .repo
            .get_by_id(shipment_id)
            .await?
            .ok_or(DomainError::ShipmentNotFoundById { id: shipment_id })?;

        // 2. Check that the user exists and is a service provider

        let user = self
            .user_repo
            .get_by_id(provider_id)
            .await?
            .ok_or(DomainError::UserNotFoundWithId { id: provider_id })?;
        if user.role() != UserRole::ServiceProvider {
            return Err(DomainError::UserIsNotServiceProvider {
                 provider_id,
            });
        }
        // 3. Assign provider and update status

        self.repo
            .assign_service_provider(shipment_id, provider_id)
            .await
            .map_err(DomainError::from)?;

        let updated_shipment = shipment.update_status(ShipmentStatus::Assigned)?;

        self.repo
            .update(&updated_shipment)
            .await
            .map_err(DomainError::from)?;
        Ok(updated_shipment)
    }
}
