use crate::domain::models::support_status::SupportStatus;
use crate::repositories::sqlx_support_repository::SqlxSupportRepository;
use crate::repositories::support_repository::SupportRepository;
use crate::tests::common::db::TestDb;
use crate::tests::common::fixtures::{test_complaint, test_shipment};

pub struct TestContext {
    pub repo: SqlxSupportRepository,
}

impl TestContext {
    pub async fn new() -> Self {
        let db = TestDb::new().await;
        let repo = SqlxSupportRepository::new(db.pool.clone());

        Self {  repo }
    }
}

#[tokio::test]
async fn should_persist_complaint() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());

    ctx.repo.persist_complaint(&complaint).await.unwrap();

    let result = ctx.repo.get_complaint_by_id(complaint.id()).await.unwrap();

    assert!(result.is_some());

    let fetched = result.unwrap();

    assert_eq!(fetched.id(), complaint.id());
}

#[tokio::test]
async fn should_get_complaints_by_status() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());

    ctx.repo.persist_complaint(&complaint).await.unwrap();

    let complaints = ctx.repo.get_complaint_by_status("Open").await.unwrap();

    assert!(!complaints.is_empty());

    assert_eq!(complaints[0].status(), complaint.status());
}

#[tokio::test]
async fn should_get_all_complaints() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();

    ctx.repo
        .persist_complaint(&test_complaint(shipment.id()))
        .await
        .unwrap();

    ctx.repo
        .persist_complaint(&test_complaint(shipment.id()))
        .await
        .unwrap();

    let complaints = ctx.repo.get_all_compalints().await.unwrap();

    assert!(!complaints.is_empty());
}

#[tokio::test]
async fn should_persist_comment() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());

    ctx.repo.persist_complaint(&complaint).await.unwrap();

    let comment = serde_json::json!({
        "message": "Issue is being investigated"
    });

    ctx.repo
        .persist_comment(complaint.id(), comment)
        .await
        .unwrap();

    let updated = ctx
        .repo
        .get_complaint_by_id(complaint.id())
        .await
        .unwrap()
        .unwrap();

    assert!(updated.comment().is_empty())
}

#[tokio::test]
async fn should_update_complaint_status() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());

    let persisted_complaint = ctx.repo.persist_complaint(&complaint).await.unwrap();
    println!("COMPLAINT IN DB = {:?}", persisted_complaint);

    let updated_status = ctx
        .repo
        .update_complaint_status(&SupportStatus::Resolved, &complaint)
        .await
        .unwrap();
    println!("Updated complaint = {:?}", updated_status);

    let updated = ctx
        .repo
        .get_complaint_by_id(complaint.id())
        .await
        .unwrap()
        .unwrap();
    println!("fetched updated complaint = {:?}", updated);

    assert_eq!(updated.status(), SupportStatus::Resolved);

    assert!(updated.resolved_at().is_some());
}

#[tokio::test]
async fn should_delete_complaint() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());

    ctx.repo.persist_complaint(&complaint).await.unwrap();

    let rows = ctx.repo.delete_complaint(complaint.id()).await.unwrap();

    assert_eq!(rows, 1);

    let result = ctx.repo.get_complaint_by_id(complaint.id()).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn should_get_complaint_by_id() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());

    // Arrange: persist
    ctx.repo.persist_complaint(&complaint).await.unwrap();

    // Act: fetch
    let result = ctx.repo.get_complaint_by_id(complaint.id()).await.unwrap();

    // Assert: exists
    assert!(result.is_some());

    let fetched = result.unwrap();

    assert_eq!(fetched.id(), complaint.id());
    assert_eq!(fetched.user_id(), complaint.user_id());
    assert_eq!(fetched.shipment_id(), complaint.shipment_id());
    assert_eq!(fetched.subject(), complaint.subject());
    assert_eq!(fetched.description(), complaint.description());
    assert_eq!(fetched.status(), complaint.status());
}
