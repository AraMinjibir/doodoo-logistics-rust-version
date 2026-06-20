use serde_json::json;
use uuid::Uuid;

use crate::tests::common::{
    db::spawn_app,
    fixtures::{comment_payload, create_complaint_payload, create_test_shipment},
};

#[tokio::test]
async fn should_successfully_send_a_new_complaint() {
    let ctx = spawn_app().await;

    let shipment_id = create_test_shipment(&ctx.app).await;

    let payload = create_complaint_payload(shipment_id);

    let req = actix_web::test::TestRequest::post()
        .uri("/complaints")
        .set_json(&payload)
        .to_request();

    let resp = actix_web::test::call_service(&ctx.app, req).await;

    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn should_successfully_add_a_comment_to_an_existing_complaint() {
    // Arrange
    let ctx = spawn_app().await;

    let shipment_id = create_test_shipment(&ctx.app).await;

    let payload = create_complaint_payload(shipment_id);

    let req = actix_web::test::TestRequest::post()
        .uri("/complaints")
        .set_json(&payload)
        .to_request();

    let setup_resp = actix_web::test::call_service(&ctx.app, req).await;
    let body: serde_json::Value = actix_web::test::read_body_json(setup_resp).await;

    let complaint_id = Uuid::parse_str(body["id"].as_str().unwrap()).unwrap();

    let comment_payload = comment_payload(complaint_id);

    // Act: Target the nested sub-resource endpoint
    let req = actix_web::test::TestRequest::patch()
        .uri(&format!("/complaints/{}/comment", complaint_id))
        .set_json(&comment_payload)
        .to_request();

    let resp = actix_web::test::call_service(&ctx.app, req).await;

    // Assert
    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn should_get_complaint_by_id() {
    // Arrange
    let ctx = spawn_app().await;

    let shipment_id = create_test_shipment(&ctx.app).await;

    let payload = create_complaint_payload(shipment_id);

    let req = actix_web::test::TestRequest::post()
        .uri("/complaints")
        .set_json(&payload)
        .to_request();

    let setup_resp = actix_web::test::call_service(&ctx.app, req).await;
    let body: serde_json::Value = actix_web::test::read_body_json(setup_resp).await;

    let complaint_id = Uuid::parse_str(body["id"].as_str().unwrap()).unwrap();

    // Action: Fetch by ID
    let get_req = actix_web::test::TestRequest::get()
        .uri(&format!("/complaints/{}", complaint_id))
        .to_request();
    let get_resp = actix_web::test::call_service(&ctx.app, get_req).await;

    assert_eq!(get_resp.status(), 200);
    let fetched: serde_json::Value = actix_web::test::read_body_json(get_resp).await;
    assert_eq!(fetched["id"], complaint_id.to_string());
}

#[tokio::test]
async fn should_get_complaint_by_status() {
    let ctx = spawn_app().await;

    let shipment_id = create_test_shipment(&ctx.app).await;

    let payload = create_complaint_payload(shipment_id);

    let req = actix_web::test::TestRequest::post()
        .uri("/complaints")
        .set_json(&payload)
        .to_request();

    let setup_resp = actix_web::test::call_service(&ctx.app, req).await;
    assert_eq!(setup_resp.status(), 201);
    // Action: Fetch by Status
    let get_req = actix_web::test::TestRequest::get()
        .uri("/complaints/status/Open")
        .to_request();

    let get_resp = actix_web::test::call_service(&ctx.app, get_req).await;

    assert!(get_resp.status().is_success());

    let body: serde_json::Value = actix_web::test::read_body_json(get_resp).await;
    // At least one complaint returned
    assert!(!body.as_array().unwrap().is_empty());

    // Strict check
    assert_eq!(body[0]["status"], "Open");
}

#[tokio::test]
async fn should_get_all_complaints() {
    let ctx = spawn_app().await;

    let shipment_id = create_test_shipment(&ctx.app).await;

    let payload = create_complaint_payload(shipment_id);

    let req = actix_web::test::TestRequest::post()
        .uri("/complaints")
        .set_json(&payload)
        .to_request();

    let setup_resp = actix_web::test::call_service(&ctx.app, req).await;
    assert_eq!(setup_resp.status(), 201);

    // 3. Get all complaints
    let get_req = actix_web::test::TestRequest::get()
        .uri("/complaints")
        .to_request();

    let get_resp = actix_web::test::call_service(&ctx.app, get_req).await;

    let status = get_resp.status();
    let body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(get_resp).await).unwrap();

    assert_eq!(status, 200);
    assert!(body.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn should_update_complaint_status() {
    let ctx = spawn_app().await;

    let shipment_id = create_test_shipment(&ctx.app).await;

    let payload = create_complaint_payload(shipment_id);

    let req = actix_web::test::TestRequest::post()
        .uri("/complaints")
        .set_json(&payload)
        .to_request();

    let setup_resp = actix_web::test::call_service(&ctx.app, req).await;
    let body: serde_json::Value = actix_web::test::read_body_json(setup_resp).await;

    let complaint_id = Uuid::parse_str(body["id"].as_str().unwrap()).unwrap();

    let update_payload = json!({
        "status": "Resolved"
    });

    let req = actix_web::test::TestRequest::patch()
        .uri(&format!("/complaints/{}/status", complaint_id))
        .set_json(&update_payload)
        .to_request();

    let resp = actix_web::test::call_service(&ctx.app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value =
        serde_json::from_slice(&actix_web::test::read_body(resp).await).unwrap();

    assert_eq!(body["status"], "Resolved");
    assert_eq!(body["id"], complaint_id.to_string());
}

#[tokio::test]
async fn should_delete_complaint() {
    let ctx = spawn_app().await;

    let shipment_id = create_test_shipment(&ctx.app).await;

    let payload = create_complaint_payload(shipment_id);

    let req = actix_web::test::TestRequest::post()
        .uri("/complaints")
        .set_json(&payload)
        .to_request();

    let setup_resp = actix_web::test::call_service(&ctx.app, req).await;
    let body: serde_json::Value = actix_web::test::read_body_json(setup_resp).await;

    let complaint_id = Uuid::parse_str(body["id"].as_str().unwrap()).unwrap();

    let delete_req = actix_web::test::TestRequest::delete()
        .uri(&format!("/complaints/{}", complaint_id))
        .to_request();
    let delete_resp = actix_web::test::call_service(&ctx.app, delete_req).await;
    assert_eq!(delete_resp.status(), 204);
}
