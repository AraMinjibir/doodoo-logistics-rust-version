use super::*;
use uuid::Uuid;

use crate::domain::errors::repository_error::RepositoryError;
use crate::repositories::shipment_repository::ShipmentRepository;
use crate::domain::models::shipment::Shipment;
use crate::tests::common::fixtures::{test_shipment, test_proof};
use crate::domain::services::shipment_service_impl::ShipmentServiceImpl;
use crate::domain::services::shipment_service::ShipmentService;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::models::shipment_status::ShipmentStatus;
use crate::tests::common::mock_repo::MockRepo;





#[tokio::test]
async fn create_shipment_success() {
    let mut repo = MockRepo::new();
    let shipment = test_shipment();

    let expected_id = shipment.id();

    repo.expect_create()
        .withf(move |s| s.id() == expected_id)
        .times(1)
        .returning(|_| Ok(()));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.create_shipment(shipment.clone()).await;

    assert!(result.is_ok());

    let created = result.unwrap();
    assert_eq!(created.id(), shipment.id());
}

#[tokio::test]
async fn create_shipment_repo_error() {
    let mut repo = MockRepo::new();

    repo.expect_create()
        .returning(|_| Err(RepositoryError::DatabaseError("fail".into())));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.create_shipment(test_shipment()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn get_by_tracking_success() {
    let mut repo = MockRepo::new();
    let shipment = test_shipment();

    repo.expect_find_by_tracking_number()
        .returning(move |_| Ok(Some(shipment.clone())));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.get_by_tracking_number("T1").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_by_tracking_not_found() {
    let mut repo = MockRepo::new();

    repo.expect_find_by_tracking_number()
        .returning(|_| Ok(None));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.get_by_tracking_number("T1").await;

    assert!(matches!(result, Err(DomainError::ShipmentNotFound { .. })));
}

#[tokio::test]
async fn get_by_id_success() {
    let mut repo = MockRepo::new();
    let shipment = test_shipment();
    let id = shipment.id();

    repo.expect_get_by_id()
        .returning(move |_| Ok(Some(shipment.clone())));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.get_by_id(id).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_by_id_not_found() {
    let mut repo = MockRepo::new();

    repo.expect_get_by_id()
        .returning(|_| Ok(None));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.get_by_id(Uuid::new_v4()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn get_by_status_success() {
    let mut repo = MockRepo::new();

    repo.expect_get_by_status()
        .returning(|_| Ok(vec![test_shipment()]));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.get_by_status(ShipmentStatus::Created).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn update_status_success() {
    let mut repo = MockRepo::new();

    let mut shipment = test_shipment();
    shipment.set_status(ShipmentStatus::InTransit); 

    let tracking = "TRACK123".to_string();

    let tracking_match = tracking.clone();
    let shipment_find = shipment.clone();

    repo.expect_find_by_tracking_number()
        .withf(move |t| t == tracking_match)
        .returning(move |_| Ok(Some(shipment_find.clone())));

    repo.expect_update()
        .returning(|_| Ok(()));

    let service = ShipmentServiceImpl::new(repo);

    let result = service
        .update_status(&tracking, ShipmentStatus::Delivered, None)
        .await;

    match result {
        Ok(_) => println!("SUCCESS"),
        Err(e) => panic!("FAILED: {:?}", e),
    }
}

#[tokio::test]
async fn update_status_not_found() {
    let mut repo = MockRepo::new();

    repo.expect_find_by_tracking_number()
        .returning(|_| Ok(None));

    let service = ShipmentServiceImpl::new(repo);

    let result = service
        .update_status("T1", ShipmentStatus::InTransit, None)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn update_shipment_success() {
    let mut repo = MockRepo::new();

    let shipment = test_shipment();
    let shipment_id = shipment.id();

    let existing = shipment.clone();
    let updated = shipment.clone();

    repo.expect_get_by_id()
        .withf(move |id| *id == shipment_id)
        .returning(move |_| Ok(Some(existing.clone())));

    repo.expect_update()
        .returning(|_| Ok(()));

    let service = ShipmentServiceImpl::new(repo);

    let result = service
        .update_shipment(shipment_id, updated)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn list_shipments_success() {
    let mut repo = MockRepo::new();

    repo.expect_list_all()
        .returning(|_, _| Ok(vec![test_shipment()]));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.list_shipments(0, 10).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn delete_success() {
    let mut repo = MockRepo::new();

    repo.expect_delete()
        .returning(|_| Ok(1));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.delete_shipment(Uuid::new_v4()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_not_found() {
    let mut repo = MockRepo::new();

    repo.expect_delete()
        .returning(|_| Ok(0));

    let service = ShipmentServiceImpl::new(repo);

    let result = service.delete_shipment(Uuid::new_v4()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn upload_proof_success() {
    let mut repo = MockRepo::new();

    let mut shipment = test_shipment();
    shipment.set_status(ShipmentStatus::Delivered);

    let tracking = "TRACK123".to_string();

    let tracking_match = tracking.clone();
    let shipment_find = shipment.clone();
    let shipment_upload = shipment.clone(); 

    repo.expect_find_by_tracking_number()
        .withf(move |t| t == tracking_match)
        .returning(move |_| Ok(Some(shipment_find.clone())));

    repo.expect_upload_proof_of_delivery()
        .returning(move |_, _| Ok(Some(shipment_upload.clone())));

    let service = ShipmentServiceImpl::new(repo);

    let proof = test_proof();

    let result = service
        .upload_proof_of_delivery(&tracking, proof)
        .await;

        assert!(result.is_ok());
}

#[tokio::test]
async fn upload_proof_not_found() {
    let mut repo = MockRepo::new();

    repo.expect_find_by_tracking_number()
        .returning(|_| Ok(None));

    let service = ShipmentServiceImpl::new(repo);

    let result = service
        .upload_proof_of_delivery("T1", test_proof())
        .await;

    assert!(result.is_err());
}