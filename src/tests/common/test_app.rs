#![allow(dead_code)] // This tells Rust to only look at this folder during 'cargo test'

use std::sync::Arc;
use sqlx::PgPool;
use actix_web::{test, web, App};
use actix_web::dev::{Service, ServiceResponse};
use actix_http::Request;

use crate::config::app_state::AppState;
use crate::repositories::sqlx_shipment_repository::SqlxShipmentRepository;
use crate::domain::services::shipment_service_impl::ShipmentServiceImpl;
use crate::domain::services::shipment_service::ShipmentService;
use crate::configure_routes;


pub async fn setup_app_with_pool(
    pool: PgPool,
) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    let repo = SqlxShipmentRepository::new(pool);

    let service_impl = ShipmentServiceImpl::new(repo);

    let service: Arc<dyn ShipmentService + Send + Sync> =
        Arc::new(service_impl);

    let state = AppState {
        shipment_service: service,
    };

    test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await
}

