use sqlx::{PgPool, Executor};
use std::env;
use dotenvy::dotenv;

pub struct TestDb {
    pub pool: PgPool,
}


impl TestDb {

    pub async fn new() -> Self {
        dotenv().ok();
        
        // Look for DATABASE_URL, but provide a default for CI if it's missing
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@postgres:5432/doodoo_test".to_string());
    
        let pool = PgPool::connect(&database_url)
            .await
            .unwrap_or_else(|e| panic!("Failed to connect to DB at {}: {}", database_url, e));
    
    // Run migrations automatically
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");
    
    // Clean database BEFORE each test
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