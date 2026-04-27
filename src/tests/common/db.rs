use sqlx::{PgPool, Executor};
use std::env;
use dotenvy::dotenv;

pub struct TestDb {
    pub pool: PgPool,
}

impl TestDb {
    pub async fn new() -> Self {
        dotenv().ok();
        // 1. Load test DB URL
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to DB");

        // 2. Run migrations automatically
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        // 3. Clean database BEFORE each test
        Self::clean(&pool).await;

        Self { pool }
    }

    async fn clean(pool: &PgPool) {
        // ⚠️ order matters if you have FK constraints
        pool.execute("TRUNCATE TABLE shipments RESTART IDENTITY CASCADE")
            .await
            .unwrap();
    }
}