#![allow(dead_code)] // This tells Rust to only look at this folder during 'cargo test'

use sqlx::{PgPool, Executor, postgres::PgPoolOptions};
use actix_web::dev::{Service, ServiceResponse};
use actix_http::Request;
use std::env;
use dotenvy::dotenv;
use std::time::Duration;
use testcontainers::{clients::Cli, Container};
use testcontainers_modules::postgres::Postgres;
use tokio::time::sleep;
use std::sync::OnceLock;
static DOCKER: OnceLock<Cli> = OnceLock::new();

use crate::tests::common::test_app::setup_app_with_pool;


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
        // Order matters if there is FK constraints
        pool.execute("TRUNCATE TABLE shipments RESTART IDENTITY CASCADE")
            .await
            .unwrap();
    }
}

pub struct TestContainerDb<'a> {
    pub container: Container<'a, Postgres>,
    pub connection_string: String,
}

pub async fn setup_test_db<'a>(docker: &'a Cli) -> TestContainerDb<'a> {
    let container = docker.run(Postgres::default());

    let port = container.get_host_port_ipv4(5432);

    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        port
    );

    TestContainerDb {
        container,
        connection_string,
    }
}

pub async fn wait_for_db(connection_string: &str) -> PgPool {
    for _ in 0..10 {
        match PgPool::connect(connection_string).await {
            Ok(pool) => return pool,
            Err(_) => sleep(Duration::from_secs(1)).await,
        }
    }

    panic!("Database not ready");
}

pub struct TestContext<S> {
    pub app: S,
    pub pool: PgPool,
    pub _container: Container<'static, Postgres>,
}

pub async fn spawn_app() -> TestContext<impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>> {
    // 1. Initialize Docker and DB
    let docker = DOCKER.get_or_init(Cli::default);
    let container = docker.run(Postgres::default());

    let port = container.get_host_port_ipv4(5432);

    let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);
    
    // 2. Connect and Migrate
    let pool = wait_for_db(&connection_string).await;
    sqlx::migrate!("./migrations").run(&pool).await.expect("Migrations failed");
    // 3. Build the Actix App
    let app = setup_app_with_pool(pool.clone()).await;

    TestContext { 
        app, 
        pool, 
        _container: container 
    }
}