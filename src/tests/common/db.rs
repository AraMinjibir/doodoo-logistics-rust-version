use sqlx::{PgPool, Executor, postgres::PgPoolOptions};
use std::env;
use dotenvy::dotenv;
use std::time::Duration;

pub struct TestDb {
    pub pool: PgPool,
}


impl TestDb {

    pub async fn new() -> Self {
        dotenv().ok();
        
        // Look for DATABASE_URL, but provide a default for CI if it's missing
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@postgres:5432/doodoo_test".to_string());
    
        let pool = loop {
            match PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
            {
                Ok(pool) => break pool,
                Err(e) => {
                    eprintln!("Retrying DB connection: {}", e);
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        };
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