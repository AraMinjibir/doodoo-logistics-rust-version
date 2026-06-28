use crate::{
    domain::models::user_status::UserRole,
    tests::common::{
        db::spawn_app,
        fixtures::{user_signup_payload, CreatedUser},
    },
};
use actix_web::{http::StatusCode, test};
use serde_json::json;

#[tokio::test]
async fn should_signup_user() {
    let ctx = spawn_app().await;

    let req = actix_web::test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    let resp = actix_web::test::call_service(&ctx.app, req).await;
    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn should_login_user() {
    let ctx = spawn_app().await;

    // Arrange: create a user
    let signup_req = actix_web::test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    let signup_resp = actix_web::test::call_service(&ctx.app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::CREATED);

    // Act: login
    let login_payload = json!({
        "email": "doodoo@email.com",
        "password": "paSSword"
    });

    let login_req = actix_web::test::TestRequest::post()
        .uri("/users/login")
        .set_json(&login_payload)
        .to_request();

    let login_resp = actix_web::test::call_service(&ctx.app, login_req).await;

    // Assert
    assert_eq!(login_resp.status(), StatusCode::OK);

    let body = actix_web::test::read_body(login_resp).await;
    println!("{}", String::from_utf8_lossy(&body));
}

#[tokio::test]
async fn should_get_user_by_id() {
    let ctx = spawn_app().await;

    let signup_req = test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    let signup_resp = test::call_service(&ctx.app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::CREATED);

    let body = test::read_body(signup_resp).await;
    let created_user: CreatedUser = serde_json::from_slice(&body).unwrap();

    let req = test::TestRequest::get()
        .uri(&format!("/users/{}", created_user.id))
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let fetched: CreatedUser = serde_json::from_slice(&body).unwrap();

    assert_eq!(fetched.id, created_user.id);
    assert_eq!(fetched.email, created_user.email);
}

#[tokio::test]
async fn should_get_user_by_email() {
    let ctx = spawn_app().await;

    let signup_req = test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    let signup_resp = test::call_service(&ctx.app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::CREATED);

    let req = test::TestRequest::get()
        .uri("/users/email/doodoo@email.com")
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_get_users_by_status() {
    let ctx = spawn_app().await;

    let signup_req = test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    test::call_service(&ctx.app, signup_req).await;

    let req = test::TestRequest::get()
        .uri("/users/status/Active")
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let users: Vec<CreatedUser> = serde_json::from_slice(&body).unwrap();

    assert!(!users.is_empty());
}
#[tokio::test]
async fn should_get_users_by_role() {
    let ctx = spawn_app().await;

    let signup_req = test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    test::call_service(&ctx.app, signup_req).await;

    let req = test::TestRequest::get()
        .uri("/users/role/Admin")
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let users: Vec<CreatedUser> = serde_json::from_slice(&body).unwrap();

    assert_eq!(users[0].role, UserRole::Admin);
}

#[tokio::test]
async fn should_get_all_users() {
    let ctx = spawn_app().await;

    let signup_req = test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    test::call_service(&ctx.app, signup_req).await;

    let req = test::TestRequest::get().uri("/users").to_request();

    let resp = test::call_service(&ctx.app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let users: Vec<CreatedUser> = serde_json::from_slice(&body).unwrap();

    assert!(!users.is_empty());
}

#[tokio::test]
async fn should_update_user_status() {
    let ctx = spawn_app().await;

    // Arrange
    let signup_req = test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    let signup_resp = test::call_service(&ctx.app, signup_req).await;
    let body = test::read_body(signup_resp).await;
    let created_user: CreatedUser = serde_json::from_slice(&body).unwrap();
    // Act
    let req = test::TestRequest::patch()
        .uri(&format!("/users/{}/status", created_user.id))
        .set_json(json!({
            "status": "Suspended"
        }))
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    println!("{}", String::from_utf8_lossy(&body));
}

#[tokio::test]
async fn should_update_user() {
    let ctx = spawn_app().await;

    let signup_req = test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    let signup_resp = test::call_service(&ctx.app, signup_req).await;
    let body = test::read_body(signup_resp).await;
    let created_user: CreatedUser = serde_json::from_slice(&body).unwrap();

    let req = test::TestRequest::put()
        .uri(&format!("/users/{}", created_user.id))
        .set_json(json!({
            "name": "Updated Name",
            "phone_number": "08012345678"
        }))
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    println!("{}", String::from_utf8_lossy(&body));
}

#[tokio::test]
async fn should_delete_user() {
    let ctx = spawn_app().await;

    let signup_req = test::TestRequest::post()
        .uri("/users/signup")
        .set_json(&user_signup_payload())
        .to_request();

    let signup_resp = test::call_service(&ctx.app, signup_req).await;
    let body = test::read_body(signup_resp).await;
    let created_user: CreatedUser = serde_json::from_slice(&body).unwrap();

    let req = test::TestRequest::delete()
        .uri(&format!("/users/{}", created_user.id))
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}
