use crate::configure_routes;
use crate::domain::services::jwt_service::JwtService;
use crate::domain::services::support_service::SupportService;
use crate::domain::services::support_service_imp::SupportServiceImpl;
use crate::domain::services::user_service::UserService;
use crate::domain::services::user_service_impl::UserServiceImpl;
use crate::repositories::sqlx_support_repository::SqlxSupportRepository;
use crate::repositories::sqlx_user_repository::SqlxUserRepository;
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, web, App};
use sqlx::PgPool;
use std::sync::Arc;

use crate::config::app_state::AppState;
use crate::domain::services::{
    payment_service::PaymentService, payment_service_impl::PaymentServiceImpl,
    shipment_service::ShipmentService, shipment_service_impl::ShipmentServiceImpl,
};
use crate::repositories::{
    sqlx_payment_repository::SqlxPaymentRepository,
    sqlx_shipment_repository::SqlxShipmentRepository,
};

use crate::MockPaymentGateway;
use crate::PaymentGateway;

pub async fn setup_app_with_pool(
    pool: PgPool,
) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    let shipment_repo = Arc::new(SqlxShipmentRepository::new(pool.clone()));
    let payment_repo = Arc::new(SqlxPaymentRepository::new(pool.clone()));
    let gateway: Arc<dyn PaymentGateway + Send + Sync> = Arc::new(MockPaymentGateway::new());
    let support_repo = Arc::new(SqlxSupportRepository::new(pool.clone()));
    let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));
    let jwt = JwtService::new("test-secret".to_string(), 60);

    let shipment_service_impl = ShipmentServiceImpl::new(shipment_repo.clone());

    let payment_service_impl =
        PaymentServiceImpl::new(payment_repo, shipment_repo.clone(), gateway);

    let support_service_impl = SupportServiceImpl::new(support_repo.clone());

    let user_service_impl = UserServiceImpl::new(user_repo.clone(), jwt);

    let shipment_service: Arc<dyn ShipmentService + Send + Sync> = Arc::new(shipment_service_impl);

    let payment_service: Arc<dyn PaymentService + Send + Sync> = Arc::new(payment_service_impl);

    let support_service: Arc<dyn SupportService + Send + Sync> = Arc::new(support_service_impl);
    let user_service: Arc<dyn UserService + Send + Sync> = Arc::new(user_service_impl);

    let state = AppState {
        shipment_service,
        payment_service,
        support_service,
        user_service,
    };

    test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await
}
