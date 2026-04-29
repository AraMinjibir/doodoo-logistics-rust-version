use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::shipment::{self, Shipment};
use crate::domain::services::shipment_service::ShipmentService;
use crate::repositories::shipment_repository::ShipmentRepository;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::models::shipment_status::ShipmentStatus;
use crate::domain::models::proof_of_delivery::ProofOfDelivery;


pub struct ShipmentServiceImpl<R>
where
    R: ShipmentRepository,
{
    repo: R,
    
}
impl<R> ShipmentServiceImpl<R>
where
    R: ShipmentRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> ShipmentService for ShipmentServiceImpl<R>
where
    R: ShipmentRepository + Send + Sync
{
    async fn create_shipment(
        &self,
        shipment: Shipment,
    ) -> Result<Shipment, DomainError> {
        self.repo.create(&shipment).await?;

    Ok(shipment)
    }

    async fn update_status(
        &self,
        tracking: &str,
        status: ShipmentStatus,
        _location: Option<String>,
    ) -> Result<Shipment, DomainError> {
        
            let shipment = self.repo
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
        let affected = self.repo
            .delete(id)
            .await
            .map_err(DomainError::from)?;
    
        
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
        let valid_proof = ProofOfDelivery::create(
            proof.image(),
            proof.note(),
            proof.submitted_by(),
        )
        .map_err(|errs| {
            DomainError::ValidationError(
                errs.into_iter().map(|e| e.to_string()).collect()
            )
        })?;
    
        // 2. Fetch shipment
        let shipment = self.repo
            .find_by_tracking_number(tracking)
            .await?
            .ok_or(DomainError::ShipmentNotFound {
                tracking_number: tracking.to_string(),
            })?;
    
        // 3. Domain logic
        let updated = shipment
            .attach_proof_of_delivery(valid_proof)?;
    
        // 4. Serialize
        let proof_json = serde_json::to_value(updated.proof_of_delivery())?;
    
        // 5. Persist
        let saved = self.repo
            .upload_proof_of_delivery(updated.id(), proof_json)
            .await?
            .ok_or(DomainError::DuplicateProofOfDelivery)?;
    
        Ok(saved)
    }

    async fn get_by_tracking_number(
        &self,
        tracking: &str,
    ) -> Result<Option<Shipment>, DomainError> {
        let shipments = self.repo
            .find_by_tracking_number(tracking)
            .await?;
          Ok(shipments)
          }

    async fn get_by_id(
        &self,
        id: Uuid,
    ) -> Result<Shipment, DomainError> {
        let shipment = self.repo
            .get_by_id(id)
            .await?
            .ok_or(DomainError::ShipmentNotFoundById { id })?;
    
        Ok(shipment)
    }

async fn update_shipment(
    &self,
    id: Uuid,
    shipment: Shipment,
) -> Result<Shipment, DomainError> {

    let existing = self.repo
        .get_by_id(id)
        .await?
        .ok_or(DomainError::ShipmentNotFoundById { id })?;

    let updated = existing.updated_shipment(
        shipment.sender_name().to_string(),
        shipment.recipient().clone(),
        shipment.package_details().clone(),

    );

    self.repo
        .update(&updated)
        .await
?;

    Ok(updated)
}

async fn list_shipments(
    &self,
    offset: i64,
    limit: i64,
) -> Result<Vec<Shipment>, DomainError> {
    let shipments = self.repo
        .list_all(offset, limit)
        .await?;

    Ok(shipments)
}
async fn get_by_status(
    &self,
    status: ShipmentStatus,
) -> Result<Vec<Shipment>, DomainError> {
  let shipments =  self.repo
        .get_by_status(&status.to_string()) 
        .await?;
    
    Ok(shipments)
}
}