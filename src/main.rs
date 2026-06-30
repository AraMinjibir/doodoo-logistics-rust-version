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
use crate::domain::services::jwt_service::JwtService;
use crate::domain::services::payment_service_impl::PaymentServiceImpl;
use crate::domain::services::shipment_service_impl::ShipmentServiceImpl;
use crate::domain::services::support_service_imp::SupportServiceImpl;
use crate::domain::services::user_service_impl::UserServiceImpl;
use crate::repositories::sqlx_payment_repository::SqlxPaymentRepository;
use crate::repositories::sqlx_shipment_repository::SqlxShipmentRepository;
use crate::repositories::sqlx_support_repository::SqlxSupportRepository;
use crate::repositories::sqlx_user_repository::SqlxUserRepository;

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
    let support_repo = Arc::new(SqlxSupportRepository::new(pool.clone()));
    let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));
    let jwt = JwtService::new("test-secret".to_string(), 60);

    // 3. Services
    let shipment_service = ShipmentServiceImpl::new(shipment_repo.clone(), user_repo.clone());

    let payment_service = PaymentServiceImpl::new(
        payment_repo.clone(),
        shipment_repo.clone(),
        Arc::clone(&gateway),
    );
    let support_service = SupportServiceImpl::new(support_repo.clone());
    let user_service = UserServiceImpl::new(user_repo.clone(), jwt);
    // 4. App State
    let state = web::Data::new(AppState {
        shipment_service: Arc::new(shipment_service),
        payment_service: Arc::new(payment_service),
        support_service: Arc::new(support_service),
        user_service: Arc::new(user_service),
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
