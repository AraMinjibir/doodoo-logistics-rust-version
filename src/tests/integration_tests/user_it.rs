use crate::{
    domain::models::user_status::{UserRole, UserStatus},
    repositories::{sqlx_user_repository::SqlxUserRepository, user_repository::UserRepository},
    tests::common::{db::TestDb, fixtures::test_user},
};

pub struct TestContext {
    pub repo: SqlxUserRepository,
}

impl TestContext {
    pub async fn new() -> Self {
        let db = TestDb::new().await;
        TestDb::init(&db.pool).await;

        sqlx::query!("DELETE FROM users")
            .execute(&db.pool)
            .await
            .unwrap();

        let repo = SqlxUserRepository::new(db.pool.clone());

        Self { repo }
    }
}

#[tokio::test]
async fn should_create_and_fetch_user() {
    let ctx = TestContext::new().await;

    let user = test_user();

    ctx.repo.create_user(&user).await.unwrap();

    let result = ctx.repo.get_by_id(user.id()).await.unwrap();

    assert!(result.is_some());

    let fetched_user = result.unwrap();

    assert_eq!(fetched_user.id(), user.id());
    assert_eq!(fetched_user.email(), user.email());
}
#[tokio::test]
async fn should_get_user_by_status() {
    let ctx = TestContext::new().await;

    let user = test_user();

    ctx.repo.create_user(&user).await.unwrap();
    let status = user.status().to_string();

    let active_user = ctx.repo.get_by_status(&status).await.unwrap();

    assert!(!active_user.is_empty());
    assert_eq!(active_user[0].status(), UserStatus::Active);
}
#[tokio::test]
async fn should_get_user_by_role() {
    let ctx = TestContext::new().await;

    let user = test_user();

    ctx.repo.create_user(&user).await.unwrap();
    let role = user.role().to_string();

    let active_user = ctx.repo.get_by_role(&role).await.unwrap();

    assert!(!active_user.is_empty());
    assert_eq!(active_user[0].role(), UserRole::Admin);
}

#[tokio::test]
async fn should_get_all_users() {
    let ctx = TestContext::new().await;

    let user1 = test_user();
    let user2 = test_user();
    ctx.repo.create_user(&user1).await.unwrap();
    ctx.repo.create_user(&user2).await.unwrap();

    let users = ctx.repo.get_all().await.unwrap();
    println!("users: {:?}", users);

    assert!(!users.is_empty());
}

#[tokio::test]
async fn should_update_user_status() {
    let ctx = TestContext::new().await;

    let mut user = test_user();
    user.set_status(UserStatus::Suspended);

    print!("mutated user:{:?}", user);

    ctx.repo.create_user(&user).await.unwrap();

    ctx.repo.update(&user).await.unwrap();
    let updated = ctx.repo.get_by_id(user.id()).await.unwrap().unwrap();
    print!("fetched updated from DB:{:?}", updated);

    assert_eq!(updated.status(), UserStatus::Suspended)
}

#[tokio::test]
async fn should_delete_user() {
    let ctx = TestContext::new().await;

    let user = test_user();
    ctx.repo.create_user(&user).await.unwrap();

    let deleted_row = ctx.repo.delete(user.id()).await.unwrap();

    assert_eq!(deleted_row, 1);
}
