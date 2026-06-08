use crate::tests::common::db::spawn_app;
use crate::tests::common::fixtures::{create_test_shipment, generate_payment_payload};
use actix_web::test::{TestRequest, call_service};
use chrono::Datelike;

#[tokio::test]
async fn should_generate_payment() {
    let ctx = spawn_app().await;

    // 1. Create shipment

    let shipment_id = create_test_shipment(&ctx.app).await;

    // 2. Create payment using SAME ctx
    let payment_req = TestRequest::post()
        .uri("/payments")
        .set_json(&generate_payment_payload(shipment_id))
        .to_request();

    let resp = call_service(&ctx.app, payment_req).await;

    let status = resp.status();

    assert_eq!(status, 201);
}

#[tokio::test]
async fn should_get_payment_by_reference() {
    let ctx = spawn_app().await;

    // 1. Create shipment

    let shipment_id = create_test_shipment(&ctx.app).await;

    // 2. Create payment
    let payment_req = TestRequest::post()
        .uri("/payments")
        .set_json(&generate_payment_payload(shipment_id))
        .to_request();

    let payment_resp = call_service(&ctx.app, payment_req).await;
    assert_eq!(payment_resp.status(), 201);

    let payment_body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(payment_resp).await).unwrap();

    let reference = payment_body["reference_number"]
        .as_str()
        .unwrap()
        .to_string();

    // 3. Fetch payment by reference
    let get_req = TestRequest::get()
        .uri(&format!("/payments/reference/{}", reference))
        .to_request();

    let get_resp = call_service(&ctx.app, get_req).await;

    let status = get_resp.status();
    let body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(get_resp).await).unwrap();

    // 4. Assertions
    assert_eq!(status, 200);
    assert_eq!(body["reference_number"], reference);
    assert_eq!(body["shipment_id"], shipment_id.to_string());
}

#[tokio::test]
async fn should_get_payment_by_shipment_id() {
    let ctx = spawn_app().await;

    // 1. Create shipment

    let shipment_id = create_test_shipment(&ctx.app).await;

    // 2. Create payment
    let payment_req = TestRequest::post()
        .uri("/payments")
        .set_json(&generate_payment_payload(shipment_id))
        .to_request();

    let payment_resp = call_service(&ctx.app, payment_req).await;
    assert_eq!(payment_resp.status(), 201);

    // 3. Fetch by shipment_id
    let get_req = TestRequest::get()
        .uri(&format!("/payments/shipment/{}", shipment_id))
        .to_request();

    let get_resp = call_service(&ctx.app, get_req).await;

    let status = get_resp.status();
    let body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(get_resp).await).unwrap();

    assert_eq!(status, 200);
    assert_eq!(body["shipment_id"], shipment_id.to_string());
}

#[tokio::test]
async fn should_get_payments_by_status() {
    let ctx = spawn_app().await;

    // 1. Create shipment

    let shipment_id = create_test_shipment(&ctx.app).await;

    // 2. Create payment
    let payment_req = TestRequest::post()
        .uri("/payments")
        .set_json(&generate_payment_payload(shipment_id))
        .to_request();

    let payment_resp = call_service(&ctx.app, payment_req).await;
    assert_eq!(payment_resp.status(), 201);

    // 3. Fetch by status
    let get_req = TestRequest::get()
        .uri("/payments/status/Pending")
        .to_request();

    let get_resp = call_service(&ctx.app, get_req).await;

    let status = get_resp.status();
    let body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(get_resp).await).unwrap();

    assert_eq!(status, 200);

    // At least one payment returned
    assert!(!body.as_array().unwrap().is_empty());

    // Strict check
    assert_eq!(body[0]["status"], "Pending");
}

#[tokio::test]
async fn should_get_all_payments() {
    let ctx = spawn_app().await;

    // 1. Create shipment

    let shipment_id = create_test_shipment(&ctx.app).await;

    // 2. Create payment
    let payment_req = TestRequest::post()
        .uri("/payments")
        .set_json(&generate_payment_payload(shipment_id))
        .to_request();

    let payment_resp = call_service(&ctx.app, payment_req).await;
    assert_eq!(payment_resp.status(), 201);

    // 3. Get all payments
    let get_req = TestRequest::get().uri("/payments").to_request();

    let get_resp = call_service(&ctx.app, get_req).await;

    let status = get_resp.status();
    let body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(get_resp).await).unwrap();

    assert_eq!(status, 200);
    assert!(body.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn should_get_daily_revenue() {
    let ctx = spawn_app().await;

    let today = chrono::Utc::now().date_naive();

    // 1. Create shipment + payment (same pattern)

    let shipment_id = create_test_shipment(&ctx.app).await;

    let payment_req = TestRequest::post()
        .uri("/payments")
        .set_json(&generate_payment_payload(shipment_id))
        .to_request();

    let payment_resp = call_service(&ctx.app, payment_req).await;
    assert_eq!(payment_resp.status(), 201);

    // 2. Call revenue endpoint
    let req = TestRequest::get()
        .uri(&format!("/payments/revenue/daily/{}", today))
        .to_request();

    let resp = call_service(&ctx.app, req).await;

    let status = resp.status();
    let body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(resp).await).unwrap();

    assert_eq!(status, 200);

    // revenue should be numeric
    assert!(body.as_f64().is_some());
}

#[tokio::test]
async fn should_get_weekly_revenue() {
    let ctx = spawn_app().await;

    let date = chrono::Utc::now().date_naive();

    let req = TestRequest::get()
        .uri(&format!("/payments/revenue/weekly/{}", date))
        .to_request();

    let resp = call_service(&ctx.app, req).await;

    let status = resp.status();
    let body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(resp).await).unwrap();

    assert_eq!(status, 200);
    assert!(body.as_f64().is_some());
}

#[tokio::test]
async fn should_get_monthly_revenue() {
    let ctx = spawn_app().await;

    let now = chrono::Utc::now();
    let month = now.month();
    let year = now.year();

    let req = TestRequest::get()
        .uri(&format!("/payments/revenue/monthly/{}/{}", month, year))
        .to_request();

    let resp = call_service(&ctx.app, req).await;

    let status = resp.status();
    let body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(resp).await).unwrap();

    assert_eq!(status, 200);
    assert!(body.as_f64().is_some());
}

#[tokio::test]
async fn should_handle_payment_webhook() {
    let ctx = spawn_app().await;

    // 1. Create shipment

    let shipment_id = create_test_shipment(&ctx.app).await;

    // 2. Create payment
    let payment_req = TestRequest::post()
        .uri("/payments")
        .set_json(&generate_payment_payload(shipment_id))
        .to_request();

    let payment_resp = call_service(&ctx.app, payment_req).await;
    assert_eq!(payment_resp.status(), 201);

    let payment_body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(payment_resp).await).unwrap();

    let reference_number = payment_body["reference_number"]
        .as_str()
        .unwrap()
        .to_string();

    // 3. Webhook payload (simulate gateway callback)
    let webhook_payload = serde_json::json!({
        "reference": reference_number,
        "status": "success",
        "gateway_transaction_id": "txn_123456"
    });

    // 4. Call webhook endpoint
    let webhook_req = TestRequest::post()
        .uri("/payments/webhook")
        .insert_header(("x-signature", "valid_signature"))
        .set_json(&webhook_payload)
        .to_request();

    let webhook_resp = call_service(&ctx.app, webhook_req).await;

    let status = webhook_resp.status();

    let body_bytes = actix_web::body::to_bytes(webhook_resp.into_body())
        .await
        .unwrap();

    assert_eq!(status, 200);

    let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

    // 5. Verify payment updated
    assert_eq!(body["status"], "Successful");
    assert_eq!(body["reference_number"], reference_number);
}

#[tokio::test]
async fn should_delete_payment() {
    let ctx = spawn_app().await;

    // 1. Create shipment

    let shipment_id = create_test_shipment(&ctx.app).await;

    // 2. Create payment
    let payment_req = TestRequest::post()
        .uri("/payments")
        .set_json(&generate_payment_payload(shipment_id))
        .to_request();

    let payment_resp = call_service(&ctx.app, payment_req).await;
    assert_eq!(payment_resp.status(), 201);

    let payment_body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(payment_resp).await).unwrap();

    let reference_number = payment_body["reference_number"].as_str().unwrap();

    // 3. Delete payment
    let delete_req = TestRequest::delete()
        .uri(&format!("/payments/{}", reference_number))
        .to_request();

    let delete_resp = call_service(&ctx.app, delete_req).await;

    let status = delete_resp.status();
    let body_bytes = actix_web::test::read_body(delete_resp).await;

    println!("STATUS: {}", status);
    println!("BODY: {}", String::from_utf8_lossy(&body_bytes));

    assert_eq!(status, 204);

    // 4. Verify deletion (important)
    let get_req = TestRequest::get()
        .uri(&format!("/payments/{}", reference_number))
        .to_request();

    let get_resp = call_service(&ctx.app, get_req).await;

    assert_eq!(get_resp.status(), 404);
}
