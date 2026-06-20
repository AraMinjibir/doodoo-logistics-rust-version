use crate::tests::common::db::spawn_app;
use crate::tests::common::fixtures::create_shipment_payload;
use chrono::{DateTime, Utc};

// 1. CREATE (POST)

#[tokio::test]
async fn should_create_and_fetch_shipment() {
    let ctx = spawn_app().await;

    let req = actix_web::test::TestRequest::post()
        .uri("/shipments")
        .set_json(&create_shipment_payload())
        .to_request();

    let resp = actix_web::test::call_service(&ctx.app, req).await;
    assert_eq!(resp.status(), 201);
}

// 2. READ (GET)

#[tokio::test]
async fn should_fetch_shipment_by_id() {
    let ctx = spawn_app().await;

    // First, Create a shipment to get its ID
    let create_req = actix_web::test::TestRequest::post()
        .uri("/shipments")
        .set_json(&create_shipment_payload())
        .to_request();
    let create_resp = actix_web::test::call_service(&ctx.app, create_req).await;
    let created_shipment: serde_json::Value = actix_web::test::read_body_json(create_resp).await;
    let id = created_shipment["id"]
        .as_str()
        .expect("ID missing in response");

    // Action: Fetch by ID
    let get_req = actix_web::test::TestRequest::get()
        .uri(&format!("/shipments/{}", id))
        .to_request();
    let get_resp = actix_web::test::call_service(&ctx.app, get_req).await;

    assert_eq!(get_resp.status(), 200);
    let fetched: serde_json::Value = actix_web::test::read_body_json(get_resp).await;
    assert_eq!(fetched["id"], id);
}

#[tokio::test]
async fn should_list_shipments_with_pagination() {
    let ctx = spawn_app().await;

    // Seed 3 shipments
    for _ in 0..3 {
        let req = actix_web::test::TestRequest::post()
            .uri("/shipments")
            .set_json(&create_shipment_payload())
            .to_request();
        actix_web::test::call_service(&ctx.app, req).await;
    }

    // Action: Request page 1 with limit of 2
    let list_req = actix_web::test::TestRequest::get()
        .uri("/shipments?page=1&page_size=2")
        .to_request();

    let resp = actix_web::test::call_service(&ctx.app, list_req).await;
    assert_eq!(resp.status(), 200);

    let body: Vec<serde_json::Value> = actix_web::test::read_body_json(resp).await;
    assert_eq!(body.len(), 2, "Pagination should limit results to 2");
}

// 3. UPDATE (PUT / PATCH)

#[tokio::test]
async fn should_perform_partial_update() {
    let ctx = spawn_app().await;

    // Create a shipment
    let create_resp = actix_web::test::call_service(
        &ctx.app,
        actix_web::test::TestRequest::post()
            .uri("/shipments")
            .set_json(&create_shipment_payload())
            .to_request(),
    )
    .await;
    let created: serde_json::Value = actix_web::test::read_body_json(create_resp).await;
    let id = created["id"].as_str().unwrap();

    // Action: Update just the sender name (Partial Update)
    let update_payload = serde_json::json!({ "sender_name": "New Sender Name" });
    let update_req = actix_web::test::TestRequest::put()
        .uri(&format!("/shipments/{}", id))
        .set_json(&update_payload)
        .to_request();

    let update_resp = actix_web::test::call_service(&ctx.app, update_req).await;
    assert_eq!(update_resp.status(), 200);

    let updated_body: serde_json::Value = actix_web::test::read_body_json(update_resp).await;
    assert_eq!(updated_body["sender_name"], "New Sender Name");
}

// 4. DELETE (DELETE)

#[tokio::test]
async fn should_delete_shipment() {
    let ctx = spawn_app().await;

    // Create a shipment
    let create_resp = actix_web::test::call_service(
        &ctx.app,
        actix_web::test::TestRequest::post()
            .uri("/shipments")
            .set_json(&create_shipment_payload())
            .to_request(),
    )
    .await;
    let body: serde_json::Value = actix_web::test::read_body_json(create_resp).await;
    let id = body["id"].as_str().unwrap();

    // Action: Delete
    let delete_req = actix_web::test::TestRequest::delete()
        .uri(&format!("/shipments/{}", id))
        .to_request();
    let delete_resp = actix_web::test::call_service(&ctx.app, delete_req).await;
    assert_eq!(delete_resp.status(), 204);

    // Verification: Get should now return 404
    let get_resp = actix_web::test::call_service(
        &ctx.app,
        actix_web::test::TestRequest::get()
            .uri(&format!("/shipments/{}", id))
            .to_request(),
    )
    .await;
    assert_eq!(get_resp.status(), 404);
}

// 5. BUSINESS LOGIC & DOMAIN RULES (The "Sad Path")

#[tokio::test]
async fn should_transition_shipment_status() {
    let ctx = spawn_app().await;

    // 1. Setup
    let create_resp = actix_web::test::call_service(
        &ctx.app,
        actix_web::test::TestRequest::post()
            .uri("/shipments")
            .set_json(&create_shipment_payload())
            .to_request(),
    )
    .await;
    let create_body: serde_json::Value = actix_web::test::read_body_json(create_resp).await;
    let tracking = create_body["tracking_number"].as_str().unwrap();

    // 2. Action: Transition to InTransit
    let update_req = actix_web::test::TestRequest::patch()
        .uri(&format!("/shipments/tracking/{}/status", tracking))
        .set_json(&serde_json::json!({ "status": "InTransit" }))
        .to_request();

    let update_resp = actix_web::test::call_service(&ctx.app, update_req).await;
    assert_eq!(update_resp.status(), 200);

    // 3. Verify: Check status and timestamp update
    let update_body: serde_json::Value = actix_web::test::read_body_json(update_resp).await;
    let created_at: DateTime<Utc> = create_body["created_at"].as_str().unwrap().parse().unwrap();
    let updated_at: DateTime<Utc> = update_body["updated_at"].as_str().unwrap().parse().unwrap();

    assert_eq!(update_body["status"], "InTransit");
    assert!(updated_at >= created_at);
}

#[tokio::test]
async fn should_fail_when_transitioning_directly_from_created_to_delivered() {
    let ctx = spawn_app().await;

    // Create (Initial: Created)
    let create_resp = actix_web::test::call_service(
        &ctx.app,
        actix_web::test::TestRequest::post()
            .uri("/shipments")
            .set_json(&create_shipment_payload())
            .to_request(),
    )
    .await;
    let create_body: serde_json::Value = actix_web::test::read_body_json(create_resp).await;
    let tracking = create_body["tracking_number"].as_str().unwrap();

    // Action: Attempt illegal jump (Created -> Delivered)
    let illegal_req = actix_web::test::TestRequest::patch()
        .uri(&format!("/shipments/tracking/{}/status", tracking))
        .set_json(&serde_json::json!({ "status": "Delivered" }))
        .to_request();

    let illegal_resp = actix_web::test::call_service(&ctx.app, illegal_req).await;
    assert_eq!(illegal_resp.status(), 400); // Bad Request
}

#[tokio::test]
async fn should_upload_proof_of_delivery() {
    let ctx = spawn_app().await;

    // 1. Setup: Create and move to Delivered
    let create_resp = actix_web::test::call_service(
        &ctx.app,
        actix_web::test::TestRequest::post()
            .uri("/shipments")
            .set_json(&create_shipment_payload())
            .to_request(),
    )
    .await;
    let created: serde_json::Value = actix_web::test::read_body_json(create_resp).await;
    let tracking = created["tracking_number"].as_str().unwrap();

    // Lifecycle Walkthrough
    let states = vec!["InTransit", "OutForDelivery", "Delivered"];
    for state in states {
        let req = actix_web::test::TestRequest::patch()
            .uri(&format!("/shipments/tracking/{}/status", tracking))
            .set_json(&serde_json::json!({ "status": state }))
            .to_request();
        let resp = actix_web::test::call_service(&ctx.app, req).await;
        assert!(resp.status().is_success());
    }

    // 2. Action: Upload Proof
    let proof_payload = serde_json::json!({
        "image": "https://s3.bucket/sig.png",
        "note": "Package received with thanks",
        "submitted_by": "Ara Minjibir",
        "submitted_at": "2024-05-06T12:00:00Z",
    });

    let req_proof = actix_web::test::TestRequest::post()
        .uri(&format!("/shipments/tracking/{}/proof", tracking))
        .set_json(&proof_payload)
        .to_request();

    let resp_proof = actix_web::test::call_service(&ctx.app, req_proof).await;
    assert_eq!(resp_proof.status(), 200);
}

#[tokio::test]
async fn should_fail_upload_proof_if_status_is_not_delivered() {
    let ctx = spawn_app().await;

    // Create (Status: Created)
    let create_resp = actix_web::test::call_service(
        &ctx.app,
        actix_web::test::TestRequest::post()
            .uri("/shipments")
            .set_json(&create_shipment_payload())
            .to_request(),
    )
    .await;
    let created: serde_json::Value = actix_web::test::read_body_json(create_resp).await;
    let tracking = created["tracking_number"].as_str().unwrap();

    let proof_payload = serde_json::json!({
        "image": "https://s3.bucket/sig.png",
        "note": "Illegal upload attempt",
        "submitted_by": "Bad Actor",
        "submitted_at": "2024-05-06T12:00:00Z",
    });

    // Action: Attempt proof upload while only "Created"
    let req_created = actix_web::test::TestRequest::post()
        .uri(&format!("/shipments/tracking/{}/proof", tracking))
        .set_json(&proof_payload)
        .to_request();

    let resp_created = actix_web::test::call_service(&ctx.app, req_created).await;
    assert_eq!(resp_created.status(), 400);

    // Finalize cleanup
    ctx.pool.close().await;
}
