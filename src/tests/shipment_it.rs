use sqlx::PgPool;
use uuid::Uuid;

use crate::repositories::shipment_repository::ShipmentRepository;
use crate::repositories::sqlx_shipment_repository::SqlxShipmentRepository;
use crate::tests::common::db::TestDb;
use crate::tests::common::fixtures::test_shipment;
use crate::domain::models::shipment_status::ShipmentStatus;


pub struct TestContext {
    pub db: TestDb,
    pub repo: SqlxShipmentRepository,
}

impl TestContext {
    pub async fn new() -> Self {
        let db = TestDb::new().await;

        let repo = SqlxShipmentRepository::new(db.pool.clone());

        Self { db, repo }
    }
}

#[tokio::test]
async fn should_create_and_fetch_shipment() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();

    ctx.repo.create(&shipment).await.unwrap();

    let result = ctx.repo.get_by_id(shipment.id()).await.unwrap();

    assert!(result.is_some());

    let fetched = result.unwrap();

    assert_eq!(fetched.id(), shipment.id());
    assert_eq!(fetched.sender_name(), shipment.sender_name());
}

#[tokio::test]
async fn should_update_shipment() {
    let ctx = TestContext::new().await;

    let mut shipment = test_shipment();

    ctx.repo.create(&shipment).await.unwrap();

    shipment.set_sender_name("Updated Name".to_string());

    ctx.repo.update(&shipment).await.unwrap();

    let updated = ctx.repo.get_by_id(shipment.id()).await.unwrap().unwrap();

    assert_eq!(updated.sender_name(), "Updated Name");
}

#[tokio::test]
async fn should_delete_shipment() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();

    ctx.repo.create(&shipment).await.unwrap();

    let rows = ctx.repo.delete(shipment.id()).await.unwrap();

    assert_eq!(rows, 1);

    let result = ctx.repo.get_by_id(shipment.id()).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn should_find_by_tracking_number() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();

    ctx.repo.create(&shipment).await.unwrap();

    let result = ctx
        .repo
        .find_by_tracking_number("TRACK-123")
        .await
        .unwrap();

        let shipment = test_shipment();
        let result = ctx.repo.find_by_tracking_number(shipment.tracking_number()).await;
}

#[tokio::test]
async fn should_get_by_status() {
    let ctx = TestContext::new().await;

    let mut shipment = test_shipment();
    shipment.set_status(ShipmentStatus::Created);

    ctx.repo.create(&shipment).await.unwrap();

    let result = ctx
        .repo
        .get_by_status(&ShipmentStatus::Created.to_string())
        .await
        .unwrap();

    assert!(!result.is_empty());
    assert_eq!(result[0].status(), ShipmentStatus::Created);
}

#[tokio::test]
async fn should_list_all_with_pagination() {
    let ctx = TestContext::new().await;

    for _ in 0..5 {
        let shipment = test_shipment();
        ctx.repo.create(&shipment).await.unwrap();
    }

    let result = ctx.repo.list_all(0, 3).await.unwrap();

    assert_eq!(result.len(), 3);
}

#[tokio::test]
async fn should_upload_proof_of_delivery() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    ctx.repo.create(&shipment).await.unwrap();

    let proof = serde_json::json!([
        {
            "image": "https://example.com/proof.jpg",
            "note": "delivered",
            "submitted_by": "Ara",
            "submitted_at": chrono::Utc::now()
        }
    ]);

    let updated = ctx
        .repo
        .upload_proof_of_delivery(shipment.id(), proof.clone())
        .await
        .unwrap()
        .unwrap();

        let proofs = updated.proof_of_delivery();

        assert_eq!(proofs.len(), 1);
        
        assert_eq!(
            proofs[0].image_url(),
            Some("https://example.com/proof.jpg")
        );
    }

    #[tokio::test]
async fn should_assign_service_provider() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    let provider_id = Uuid::new_v4();

    ctx.repo.create(&shipment).await.unwrap();

    ctx.repo
        .assign_service_provider(shipment.id(), provider_id)
        .await
        .unwrap();

    let updated = ctx.repo.get_by_id(shipment.id()).await.unwrap().unwrap();

    assert_eq!(updated.service_provider_id(), Some(provider_id));
}

#[tokio::test]
async fn should_find_by_service_provider() {
    let ctx = TestContext::new().await;

    let provider_id = Uuid::new_v4();

    let mut shipment = test_shipment();
    shipment.set_service_provider_id(Some(provider_id));

    ctx.repo.create(&shipment).await.unwrap();

    let result = ctx
        .repo
        .find_by_service_provider(provider_id)
        .await
        .unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].service_provider_id(), Some(provider_id));
}