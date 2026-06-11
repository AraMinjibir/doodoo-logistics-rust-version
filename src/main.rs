use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::sync::Arc;

mod config;
mod controllers;
mod domain;
mod infrastructure;
mod repositories;
mod tests;

use crate::config::app_state::AppState;
use crate::config::routes::configure_routes;
use crate::domain::gateways::{mock_payment::MockPaymentGateway, payment_gateway::PaymentGateway};
use crate::domain::services::payment_service_impl::PaymentServiceImpl;
use crate::domain::services::shipment_service_impl::ShipmentServiceImpl;
use crate::repositories::sqlx_payment_repository::SqlxPaymentRepository;
use crate::repositories::sqlx_shipment_repository::SqlxShipmentRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // tracing
    tracing_subscriber::fmt::init();

    // 1. Load DB connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    // 2. Repositories
    let shipment_repo = Arc::new(SqlxShipmentRepository::new(pool.clone()));

    let payment_repo = Arc::new(SqlxPaymentRepository::new(pool.clone()));
    let gateway: Arc<dyn PaymentGateway + Send + Sync> = Arc::new(MockPaymentGateway::new());

    // 3. Services
    let shipment_service = ShipmentServiceImpl::new(shipment_repo.clone());

    let payment_service = PaymentServiceImpl::new(
        payment_repo.clone(),
        shipment_repo.clone(),
        Arc::clone(&gateway),
    );
    // 4. App State
    let state = web::Data::new(AppState {
        shipment_service: Arc::new(shipment_service),
        payment_service: Arc::new(payment_service),
    });

    println!("Server running at http://127.0.0.1:8080");

    // 5. Start server

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    tracing::info!("🚀 Server running at http://{}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(configure_routes)
    })
    .bind(bind_addr)?
    .run()
    .await
}
