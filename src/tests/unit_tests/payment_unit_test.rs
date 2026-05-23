use chrono::Utc;
use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::{domain_error::DomainError, repository_error::RepositoryError};
use crate::domain::gateways::{
    payment_gateway::PaymentGatewayResponse, payment_gateway::PaymentWebhookEvent,
};
use crate::domain::models::payment_status::PaymentStatus;
use crate::domain::services::payment_service::PaymentService;
use crate::domain::services::payment_service_impl::PaymentServiceImpl;
use crate::tests::common::fixtures::{test_command, test_payment, test_shipment, test_success_payment};
use crate::tests::common::mock_repo::{MockPayment, MockPaymentRepo, MockShipmentRepo};

#[tokio::test]
async fn generate_payment_sucess() {
    let mut repo = MockPaymentRepo::new();
    let mut shipment_repo = MockShipmentRepo::new();
    let mut payment_gateway = MockPayment::new();

    let shipment = test_shipment();

    let payment = test_command(shipment.id());
    let expected_reference = "mock-ref".to_string();

    shipment_repo
        .expect_get_by_id()
        .returning(move |_| Ok(Some(shipment.clone())));

    repo.expect_get_payment_by_shipment_id()
        .returning(move |_| Ok(None));

    payment_gateway.expect_initiate_payment().returning(|_| {
        Ok(PaymentGatewayResponse {
            reference: "mock-ref".to_string(),
            authorization_url: "https://mock-payments.local/auth".to_string(),
        })
    });

    repo.expect_persist_payment()
        .withf(move |payment| payment.reference_number() == expected_reference)
        .times(1)
        .returning(|_| Ok(()));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let payment_result = service.generate_payment(&payment.clone()).await;

    assert!(payment_result.is_ok());
    let paid = payment_result.unwrap();
    assert_eq!(paid.payment.reference_number(), "mock-ref");
}

#[tokio::test]
async fn generate_paymnent_failure() {
    let mut repo = MockPaymentRepo::new();
    let mut shipment_repo = MockShipmentRepo::new();
    let mut payment_gateway = MockPayment::new();

    let shipment = test_shipment();

    let payment = test_command(shipment.id());

    shipment_repo
        .expect_create()
        .returning(|_| Err(RepositoryError::DatabaseError("fail".into())));

    shipment_repo.expect_get_by_id().returning(|_| {
        Err(RepositoryError::DatabaseError(
            "ShipmentNotFound".to_string(),
        ))
    });

    repo.expect_get_payment_by_shipment_id().returning(|_| {
        Err(RepositoryError::DatabaseError(
            "ShipmentNotFound".to_string(),
        ))
    });

    payment_gateway.expect_initiate_payment().returning(|_| {
        Err(DomainError::from(RepositoryError::DatabaseError(
            "Unable to initiate the payment".to_string(),
        )))
    });

    repo.expect_persist_payment().returning(|_| {
        Err(RepositoryError::DatabaseError(
            "Fail to persist payment".to_string(),
        ))
    });

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let payment_result = service.generate_payment(&payment.clone()).await;

    assert!(payment_result.is_err());
}

#[tokio::test]
async fn get_payment_by_ref_success() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let shipment = test_shipment();

    let payment = test_payment(shipment.id());

    repo.expect_get_payment_by_ref()
        .returning(move |_| Ok(Some(payment.clone())));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let fetched_payment = service.get_payment_by_ref("REF-DOODOO-123").await;

    assert!(fetched_payment.is_ok());
}

#[tokio::test]
async fn get_payment_by_shipment_id() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let shipment = test_shipment();

    let payment = test_payment(shipment.id());

    repo.expect_get_payment_by_shipment_id()
        .returning(move |_| Ok(Some(payment.clone())));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let fetched_payment = service.get_payment_by_shipment_id(shipment.id()).await;

    assert!(fetched_payment.is_ok());
}

#[tokio::test]
async fn get_payment_by_ref_not_found() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let shipment = test_shipment();
    let payment = test_payment(shipment.id());
    let reference_number = payment.reference_number();

    repo.expect_get_payment_by_ref().returning(|_| Ok(None));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let result = service.get_payment_by_ref(&reference_number).await;

    assert!(matches!(result, Err(DomainError::PaymentNotFound { .. })));
}

#[tokio::test]
async fn get_payment_by_status() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let shipment = test_shipment();

    let payment = test_payment(shipment.id());

    repo.expect_get_payment_by_status()
        .returning(move |_| Ok(vec![payment.clone()]));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let fetched_payments = service.get_payment_by_status("Successful").await;

    assert!(fetched_payments.is_ok());
    assert_eq!(fetched_payments.unwrap().len(), 1);
}

#[tokio::test]
async fn get_all_payments() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let shipment = test_shipment();

    let payment = test_payment(shipment.id());

    repo.expect_get_all_payments()
        .returning(move || Ok(vec![payment.clone()]));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let fetched_payments = service.get_all_payments().await;

    assert!(fetched_payments.is_ok());
    assert_eq!(fetched_payments.unwrap().len(), 1);
}


#[tokio::test]
async fn get_daily_revenue() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let date = NaiveDate::from_num_days_from_ce(1);

    repo.expect_get_daily_revenue()
        .returning(|_| Ok(Some(Decimal::new(100, 2))));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let daily_revenue = service.get_daily_revenue(date).await;

    assert!(daily_revenue.is_ok());
}

#[tokio::test]
async fn get_weekly_revenue() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let date = NaiveDate::from_num_days_from_ce(1);

    repo.expect_get_weekly_revenue()
        .returning(|_| Ok(Some(Decimal::new(100, 2))));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let weekly_revenue = service.get_weekly_revenue(date).await;

    assert!(weekly_revenue.is_ok());
}

#[tokio::test]
async fn get_monthly_revenue() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let date = NaiveDate::from_num_days_from_ce(1);
    let year = date.year() as u32;
    let month = date.month() as u32;

    repo.expect_get_monthly_revenue()
        .returning(|_, _| Ok(Some(Decimal::new(100, 2))));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);

    let monthly_revenue = service.get_monthly_revenue(year, month).await;

    assert!(monthly_revenue.is_ok());
}

#[tokio::test]
async fn handle_webhook_success() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let mut gateway = MockPayment::new();

    let shipment = test_shipment();

    gateway.expect_verify_webhook().returning(|_, _| Ok(()));

    let payment = test_payment(shipment.id());
    repo.expect_get_payment_by_ref()
        .returning(move |_| Ok(Some(payment.clone())));

    repo.expect_update_payment()
        .withf(|payment| payment.status() == PaymentStatus::Successful)
        .times(1)
        .returning(|_| Ok(()));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let gateway = Arc::new(gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, gateway);

    let event = PaymentWebhookEvent {
        reference: "mock-ref".to_string(),
        status: "success".to_string(),
        gateway_transaction_id: Some("txn-123".to_string()),
    };

    let result = service.handle_webhook(&event, "bad-signature").await;

    assert!(result.is_ok());

    let updated = result.unwrap();

    assert_eq!(updated.status(), PaymentStatus::Successful);
}

#[tokio::test]
async fn handle_webhook_invalid_signature() {
    let repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let mut gateway = MockPayment::new();

    gateway.expect_verify_webhook().times(1).returning(|_, _| {
        Err(DomainError::PaymentGatewayError {
            signature: "Invalid signature".to_string(),
        })
    });

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let gateway = Arc::new(gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, gateway);

    let event = PaymentWebhookEvent {
    reference: "mock-ref".to_string(),
    status: "success".to_string(),
    gateway_transaction_id: Some("txn-123".to_string()),
};

let result = service
    .handle_webhook(&event, "bad-signature")
    .await;

    assert!(matches!(
        result,
        Err(DomainError::PaymentGatewayError { .. })
    ));
}
#[tokio::test]
async fn handle_webhook_payment_not_found() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let mut gateway = MockPayment::new();

    gateway.expect_verify_webhook().returning(|_, _| Ok(()));

    repo.expect_get_payment_by_ref()
        .times(1)
        .returning(|_| Ok(None));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let gateway = Arc::new(gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, gateway);

    let event = PaymentWebhookEvent {
        reference: "mock-ref".to_string(),
        status: "success".to_string(),
        gateway_transaction_id: Some("txn-123".to_string()),
    };

    let result = service.handle_webhook(&event, "bad-signature").await;

    assert!(matches!(result, Err(DomainError::PaymentNotFound { .. })));
}

#[tokio::test]
async fn handle_webhook_invalid_transition() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let mut gateway = MockPayment::new();

    let payment = test_success_payment(Uuid::new_v4(), Decimal::new(1000, 0), Utc::now());

    gateway.expect_verify_webhook().returning(|_, _| Ok(()));

    let payment_clone = payment.clone();

    repo.expect_get_payment_by_ref()
        .returning(move |_| Ok(Some(payment_clone.clone())));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let gateway = Arc::new(gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, gateway);

    let event = PaymentWebhookEvent {
        reference: "mock-ref".to_string(),
        status: "success".to_string(),
        gateway_transaction_id: Some("txn-123".to_string()),
    };

    let result = service.handle_webhook(&event, "bad-signature").await;

    assert!(matches!(
        result,
        Err(DomainError::InvalidPaymentStatusTransition { .. })
    ));
}

#[tokio::test]
async fn handle_webhook_update_failure() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let mut gateway = MockPayment::new();

    let payment = test_payment(uuid::Uuid::new_v4());

    gateway.expect_verify_webhook().returning(|_, _| Ok(()));

    let payment_clone = payment.clone();

    repo.expect_get_payment_by_ref()
        .returning(move |_| Ok(Some(payment_clone.clone())));

    repo.expect_update_payment()
        .times(1)
        .returning(|_| Err(RepositoryError::DatabaseError("DB unavailable".to_string())));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let gateway = Arc::new(gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, gateway);

    let event = PaymentWebhookEvent {
        reference: "mock-ref".to_string(),
        status: "success".to_string(),
        gateway_transaction_id: Some("txn-123".to_string()),
    };

    let result = service.handle_webhook(&event, "bad-signature").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn delete_payment_success() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let shipment = test_shipment();

    let payment = test_payment(shipment.id());
    let id = &payment.reference_number();

    repo.expect_get_payment_by_ref()
        .returning(move |_| Ok(Some(payment.clone())));

    repo.expect_delete_payment().returning(|_| Ok(()));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);
    let deleted_payment = service.delete_payment(id).await;

    assert!(deleted_payment.is_ok());
}

#[tokio::test]
async fn delete_payment_not_found() {
    let mut repo = MockPaymentRepo::new();
    let shipment_repo = MockShipmentRepo::new();
    let payment_gateway = MockPayment::new();

    let shipment = test_shipment();

    let payment = test_payment(shipment.id());
    let id = &payment.reference_number();

    repo.expect_get_payment_by_ref()
        .returning(move |_| Ok(None));

    repo.expect_delete_payment().returning(|_| Ok(()));

    let repo = Arc::new(repo);
    let shipment_repo = Arc::new(shipment_repo);
    let payment_gateway = Arc::new(payment_gateway);

    let service = PaymentServiceImpl::new(repo, shipment_repo, payment_gateway);
    let deleted_payment = service.delete_payment(id).await;

    assert!(deleted_payment.is_err());
}
