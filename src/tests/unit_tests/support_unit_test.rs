use std::sync::Arc;

use crate::{
    domain::{
        models::support_status::SupportStatus,
        services::{support_service::SupportService, support_service_imp::SupportServiceImpl},
    },
    tests::common::{
        fixtures::{test_comment, test_complaint, test_shipment},
        mock_repo::MockSupportRepo,
    },
};

#[tokio::test]
async fn create_complaint() {
    let mut repo = MockSupportRepo::new();
    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());

    let expected_id = complaint.id();

    repo.expect_persist_complaint()
        .withf(move |c| c.id() == expected_id)
        .times(1)
        .returning(|_| Ok(()));

    let repo = Arc::new(repo);
    let complaint_service = SupportServiceImpl::new(repo);

    let result = complaint_service.send_complaint(&complaint).await;

    assert!(result.is_ok());

    let sent = result.unwrap();
    assert_eq!(sent.id(), complaint.id());
}
#[tokio::test]
async fn create_comment() {
    let mut repo = MockSupportRepo::new();
    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());
    let comment = test_comment(complaint.id());

    let complaint_id = complaint.id();

    repo.expect_get_complaint_by_id()
        .returning(move |_| Ok(Some(complaint.clone())));

    repo.expect_persist_comment()
        .withf(move |id, _comment| *id == complaint_id)
        .times(1)
        .returning(|_, _| Ok(()));

    let repo = Arc::new(repo);
    let service = SupportServiceImpl::new(repo);

    let new_comment = service.send_comment(complaint_id, comment).await;

    assert!(new_comment.is_ok());

    let updated = new_comment.unwrap();

    assert_eq!(updated.id(), complaint_id);
}
#[tokio::test]
async fn get_complaint_by_id() {
    let mut repo = MockSupportRepo::new();
    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());
    let id = complaint.id();

    repo.expect_get_complaint_by_id()
        .returning(move |_| Ok(Some(complaint.clone())));

    let repo = Arc::new(repo);
    let service = SupportServiceImpl::new(repo);

    let fetched_complaint = service.get_complaint_by_id(id).await;
    assert!(fetched_complaint.is_ok());
}

#[tokio::test]
async fn get_complaint_by_status() {
    let mut repo = MockSupportRepo::new();
    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());

    let status = SupportStatus::Open;

    repo.expect_get_complaint_by_status()
        .returning(move |_| Ok(vec![complaint.clone()]));

    let repo = Arc::new(repo);
    let service = SupportServiceImpl::new(repo);

    let fetched = service.get_complaint_by_status(&status).await;
    assert!(fetched.is_ok());
}

#[tokio::test]
async fn get_all_complaints() {
    let mut repo = MockSupportRepo::new();
    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());

    repo.expect_get_all_compalints()
        .returning(move || Ok(vec![complaint.clone()]));

    let repo = Arc::new(repo);

    let service = SupportServiceImpl::new(repo);

    let complaints = service.get_all_compalints().await;

    assert!(complaints.is_ok());
    assert_eq!(complaints.unwrap().len(), 1);
}

#[tokio::test]
async fn delete_complaint() {
    let mut repo = MockSupportRepo::new();
    let shipment = test_shipment();
    let complaint = test_complaint(shipment.id());
    let id = complaint.id();

    repo.expect_delete_complaint().returning(|_| Ok(1));

    let repo = Arc::new(repo);
    let service = SupportServiceImpl::new(repo);

    let deleted_row = service.delete_complaint(id).await;

    assert!(deleted_row.is_ok());
}
