use std::sync::Arc;

use mockall::predicate::eq;
use uuid::Uuid;

use crate::{
    domain::{
        errors::domain_error::DomainError,
        models::{
            user::{User, UserInput},
            user_status::{UserRole, UserStatus},
        },
        services::{
            jwt_service::JwtService, user_service::UserService, user_service_impl::UserServiceImpl,
        },
    },
    tests::common::{fixtures::test_user, mock_repo::MockUserRepo},
};

#[tokio::test]
async fn register_user_success() {
    let mut repo = MockUserRepo::new();
    let user = test_user();

    repo.expect_get_by_email().returning(move |_| Ok(None));

    repo.expect_create_user()
        .times(1)
        .withf(|u| u.status() == UserStatus::Active && !u.hash_password().is_empty())
        .returning(|_| Ok(()));

    let repo = Arc::new(repo);
    let jwt_service = JwtService::new("test-secret".to_string(), 60);
    let user_service = UserServiceImpl::new(repo, jwt_service);
    let result = user_service.register_user(user.clone()).await;

    assert!(result.is_ok());

    let registered_user = result.unwrap();

    assert_eq!(registered_user.email(), user.email());
    assert_eq!(registered_user.name(), user.name());
    assert_eq!(registered_user.role(), user.role());

    assert_ne!(registered_user.id(), user.id());
}

#[tokio::test]
async fn login_success() {
    let mut repo = MockUserRepo::new();

    let password = "password123";

    let hash = User::hash_password_value(password.to_string()).unwrap();

    let user = User::create_user(
        "User".to_string(),
        "user@email.com".to_string(),
        hash,
        "08012345678".to_string(),
        UserRole::Admin,
    )
    .unwrap();

    let expected_email = user.email();

    repo.expect_get_by_email()
        .withf(move |email| *email == expected_email)
        .times(1)
        .returning(move |_| Ok(Some(user.clone())));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service
        .login("user@email.com".to_string(), password.to_string())
        .await;

    assert!(result.is_ok());

    let token = result.unwrap();

    assert!(!token.is_empty());
}

#[tokio::test]
async fn login_should_fail_when_user_not_found() {
    let mut repo = MockUserRepo::new();

    repo.expect_get_by_email().times(1).returning(|_| Ok(None));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service
        .login("missing@email.com".to_string(), "password".to_string())
        .await;

    assert!(matches!(result, Err(DomainError::UserNotFound { .. })));
}

#[tokio::test]
async fn login_should_fail_when_user_is_inactive() {
    let mut repo = MockUserRepo::new();

    let hash = User::hash_password_value("password123".to_string()).unwrap();

    let user = User::create_user(
        "User".to_string(),
        "user@email.com".to_string(),
        hash,
        "08012345678".to_string(),
        UserRole::Admin,
    )
    .unwrap();

    let user = user.update_status(UserStatus::Suspended).unwrap();

    repo.expect_get_by_email()
        .returning(move |_| Ok(Some(user.clone())));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service
        .login("user@email.com".to_string(), "password123".to_string())
        .await;

    assert!(matches!(
        result,
        Err(DomainError::UserStatusIsNotActive { .. })
    ));
}

#[tokio::test]
async fn login_should_fail_when_password_is_invalid() {
    let mut repo = MockUserRepo::new();

    let hash = User::hash_password_value("correct-password".to_string()).unwrap();

    let user = User::create_user(
        "User".to_string(),
        "user@email.com".to_string(),
        hash,
        "08012345678".to_string(),
        UserRole::Admin,
    )
    .unwrap();

    repo.expect_get_by_email()
        .returning(move |_| Ok(Some(user.clone())));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service
        .login("user@email.com".to_string(), "wrong-password".to_string())
        .await;

    assert!(matches!(result, Err(DomainError::InvalidCredentials)));
}

#[tokio::test]
async fn get_by_id_success() {
    let mut repo = MockUserRepo::new();

    let user = test_user();
    let user_id = user.id();

    repo.expect_get_by_id()
        .with(eq(user_id))
        .times(1)
        .returning(move |_| Ok(Some(user.clone())));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.get_by_id(user_id).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().id(), user_id);
}

#[tokio::test]
async fn get_by_id_should_fail_when_user_not_found() {
    let mut repo = MockUserRepo::new();

    let user_id = Uuid::new_v4();

    repo.expect_get_by_id()
        .with(eq(user_id))
        .times(1)
        .returning(|_| Ok(None));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.get_by_id(user_id).await;

    assert!(matches!(
        result,
        Err(DomainError::UserNotFoundWithId { .. })
    ));
}
#[tokio::test]
async fn get_by_status_success() {
    let mut repo = MockUserRepo::new();

    let users = vec![test_user(), test_user()];

    repo.expect_get_by_status()
        .with(eq("Active"))
        .times(1)
        .returning(move |_| Ok(users.clone()));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.get_by_status("Active").await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[tokio::test]
async fn get_by_role_success() {
    let mut repo = MockUserRepo::new();

    let users = vec![test_user()];

    repo.expect_get_by_role()
        .with(eq("Admin"))
        .times(1)
        .returning(move |_| Ok(users.clone()));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.get_by_role("Admin").await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn get_all_success() {
    let mut repo = MockUserRepo::new();

    let users = vec![test_user(), test_user(), test_user()];

    repo.expect_get_all()
        .times(1)
        .returning(move || Ok(users.clone()));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.get_all().await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 3);
}

#[tokio::test]
async fn delete_success() {
    let mut repo = MockUserRepo::new();

    let user_id = Uuid::new_v4();

    repo.expect_delete()
        .with(eq(user_id))
        .times(1)
        .returning(|_| Ok(1));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.delete(user_id).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_should_fail_when_user_not_found() {
    let mut repo = MockUserRepo::new();

    let user_id = Uuid::new_v4();

    repo.expect_delete()
        .with(eq(user_id))
        .times(1)
        .returning(|_| Ok(0));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.delete(user_id).await;

    assert!(matches!(
        result,
        Err(DomainError::UserNotFoundWithId { .. })
    ));
}
#[tokio::test]
async fn update_user_success() {
    let mut repo = MockUserRepo::new();

    let user = test_user();
    let user_id = user.id();

    let payload = UserInput {
        name: Some("Updated Name".to_string()),
        email: None,
        phone_number: None,
        role: None,
    };

    let fetched_user = user.clone();

    repo.expect_get_by_id()
        .with(eq(user_id))
        .times(1)
        .returning(move |_| Ok(Some(fetched_user.clone())));

    repo.expect_update().times(1).returning(|_| Ok(()));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.update_user(user_id, payload).await;

    assert!(result.is_ok());

    let updated = result.unwrap();

    assert_eq!(updated.name(), "Updated Name");
}
#[tokio::test]
async fn update_user_should_fail_when_user_not_found() {
    let mut repo = MockUserRepo::new();

    let user_id = Uuid::new_v4();

    repo.expect_get_by_id().times(1).returning(|_| Ok(None));

    let payload = UserInput {
        name: None,
        email: None,
        phone_number: None,
        role: None,
    };

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.update_user(user_id, payload).await;

    assert!(matches!(
        result,
        Err(DomainError::UserNotFoundWithId { .. })
    ));
}

#[tokio::test]
async fn update_status_success() {
    let mut repo = MockUserRepo::new();

    let user = test_user();
    let user_id = user.id();

    repo.expect_get_by_id()
        .with(eq(user_id))
        .times(1)
        .returning(move |_| Ok(Some(user.clone())));

    repo.expect_update().times(1).returning(|_| Ok(()));

    let service = UserServiceImpl::new(
        Arc::new(repo),
        JwtService::new("test-secret".to_string(), 60),
    );

    let result = service.update_status(user_id, UserStatus::Suspended).await;

    assert!(result.is_ok());

    let updated = result.unwrap();

    assert_eq!(updated.status(), UserStatus::Suspended);
}
