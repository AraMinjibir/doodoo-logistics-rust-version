#![allow(dead_code)]

use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;

use crate::domain::models::{shipment, payment_status::PaymentStatus};
use crate::repositories::{payment_repository::PaymentRepository, 
    shipment_repository::ShipmentRepository, 
    sqlx_payment_repository::SqlxPaymentRepository, 
    sqlx_shipment_repository::SqlxShipmentRepository};

use crate::tests::common::{db::TestDb, fixtures::
    {test_payment, test_shipment,test_success_payment}};

#[allow(dead_code)]
pub struct TestContext {
    pub db: TestDb,
    pub shipment_repo: SqlxShipmentRepository,
    pub repo:SqlxPaymentRepository,
}

impl TestContext {
    pub async fn new() -> Self {
        let db = TestDb::new().await;

        TestDb::init(&db.pool).await;
        db.clean().await;

        let shipment_repo = SqlxShipmentRepository::new(db.pool.clone());
        let repo = SqlxPaymentRepository::new(db.pool.clone());

        Self {
            db,
            shipment_repo,
            repo,
        }
    }
}

#[tokio::test]
async fn should_generate_and_fetch_payment() {
    let ctx = TestContext::new().await;

    // 1. Create shipment 
    let shipment = test_shipment();
    ctx.shipment_repo
        .create(&shipment)
        .await
        .unwrap();

        let fetched = ctx
    .shipment_repo
    .get_by_id(shipment.id())
    .await
    .unwrap();

println!("SHIPMENT IN DB = {:?}", fetched);

    // 2. Create payment 
    let payment = test_payment(shipment.id());

    ctx.repo
        .persist_payment(&payment)
        .await
        .unwrap();

    // 3. Fetch payment
    let paid = ctx.repo
        .get_payment_by_ref(&payment.reference_number())
        .await
        .unwrap();

    assert!(paid.is_some());

    let fetched = paid.unwrap();

    assert_eq!(fetched.reference_number(), payment.reference_number());
    assert_eq!(fetched.customer_id(), payment.customer_id());
}

#[tokio::test]
async fn should_get_payment_by_status(){
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    let mut payment = test_payment(shipment.id());

   payment.set_status(PaymentStatus::Successful);

    ctx.shipment_repo.create(&shipment).await.unwrap();

    ctx.repo.persist_payment(&payment).await.unwrap();

   let updated_status =  ctx.repo.get_payment_by_status(&PaymentStatus::Successful.to_string()).await.unwrap();

   assert!(!updated_status.is_empty());
   assert_eq!(updated_status[0].status(), PaymentStatus::Successful)


}

#[tokio::test]
async fn get_payment_by_shipment_id() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    ctx.shipment_repo.create(&shipment).await.unwrap();

    let payment = test_payment(shipment.id());
    ctx.repo.persist_payment(&payment).await.unwrap();

    let paid_shipping = ctx
        .repo
        .get_payment_by_shipment_id(shipment.id())
        .await
        .unwrap();

    assert!(paid_shipping.is_some());

    let found = paid_shipping.unwrap();

    assert_eq!(found.reference_number(), payment.reference_number());
}

#[tokio::test]
async fn list_all_payments() {
    let ctx = TestContext::new().await;

    for _ in 0..5 {
        let shipment = test_shipment();
        println!("SHIPMENT ID USED = {:?}", shipment.id());

ctx.shipment_repo.create(&shipment).await.unwrap();

let db_shipment = ctx.shipment_repo.get_by_id(shipment.id()).await.unwrap();
println!("SHIPMENT IN DB = {:?}", db_shipment.is_some());

let payment = test_payment(shipment.id());
println!("PAYMENT SHIPMENT_ID = {:?}", payment.shipment_id());
        ctx.repo.persist_payment(&payment).await.unwrap();
    }

    let payments = ctx.repo.get_all_payments().await.unwrap();

    assert!(!payments.is_empty());
}

#[tokio::test]
async fn update_payment(){
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    let mut payment = test_payment(shipment.id());

    ctx.shipment_repo.create(&shipment).await.unwrap();
    ctx.repo.persist_payment(&payment).await.unwrap();

    payment.set_status(PaymentStatus::Successful);
    payment.set_failure_reason(Some("Updated reason".to_string()));
    payment.set_gateway_transaction_id(Some(("Updated id".to_string())));

    ctx.repo.update_payment(&payment).await.unwrap();

    let updated_payment = ctx.repo.get_payment_by_ref(&payment.reference_number()).await.unwrap().unwrap();

    assert_eq!(updated_payment.status(), PaymentStatus::Successful);
    assert_eq!(updated_payment.failure_reason(), Some("Updated reason".to_string()));
    assert_eq!(updated_payment.gateway_transaction_id(), Some("Updated id".to_string()));

}

#[tokio::test]
async fn delete_payment(){
    let ctx =  TestContext::new().await;

    let shipment = test_shipment();
    let payment = test_payment(shipment.id());

    ctx.shipment_repo.create(&shipment).await.unwrap();
    ctx.repo.persist_payment(&payment).await.unwrap();

    ctx.repo.delete_payment(&payment.reference_number()).await.unwrap();

    let deleted_rows = ctx.repo.get_payment_by_ref(&payment.reference_number()).await.unwrap();

    assert!(deleted_rows.is_none());    
}

#[tokio::test]
async fn should_calculate_daily_revenue() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    ctx.shipment_repo.create(&shipment).await.unwrap();

    let date = NaiveDate::from_ymd_opt(2026, 1, 8).unwrap();
    let paid_at = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2026, 1, 8)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap(),
        Utc,
    );

    let payment = test_success_payment(
        shipment.id(),
        Decimal::new(5000, 0),
        paid_at,
    );

    ctx.repo.persist_payment(&payment).await.unwrap();

    let revenue = ctx.repo.get_daily_revenue(date).await.unwrap();

        println!("DAILY REVENUE = {:?}", revenue);
            assert_eq!(revenue, Some(Decimal::new(5000, 0)));
        }

#[tokio::test]
async fn should_calculate_weekly_revenue() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    ctx.shipment_repo.create(&shipment).await.unwrap();

    let date = NaiveDate::from_ymd_opt(2026, 1, 8).unwrap();
    let paid_at = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2026, 1, 8)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap(),
        Utc,
    );

    let payment = test_success_payment(
        shipment.id(),
        Decimal::new(3000, 0),
        paid_at,
    );

    ctx.repo.persist_payment(&payment).await.unwrap();

    let revenue = ctx.repo.get_weekly_revenue(date).await.unwrap();

    assert!(revenue.unwrap() >= Decimal::new(3000, 0));
}
#[tokio::test]
async fn should_calculate_monthly_revenue() {
    let ctx = TestContext::new().await;

    let shipment = test_shipment();
    ctx.shipment_repo.create(&shipment).await.unwrap();

    let paid_at = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2026, 1, 8)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap(),
        Utc,
    );

    let payment = test_success_payment(
        shipment.id(),
        Decimal::new(10000, 0),
        paid_at,
    );

    ctx.repo.persist_payment(&payment).await.unwrap();

    let revenue = ctx.repo.get_monthly_revenue(2026, 1).await.unwrap();

    assert_eq!(revenue, Some(Decimal::new(10000, 0)));
}