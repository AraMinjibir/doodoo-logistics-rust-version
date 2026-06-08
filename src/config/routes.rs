use actix_web::web;
use crate::controllers::{payment_controller, shipment_controller, health_controller};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/health",
        web::get().to(health_controller::health),
    );

    cfg.service(
        web::scope("/shipments")
            .route("", web::post().to(shipment_controller::create_shipment))
            .route("", web::get().to(shipment_controller::list_shipments))
            .route("/{id}", web::get().to(shipment_controller::get_by_id))
            .route("/{id}", web::put().to(shipment_controller::update_shipment))
            .route("/{id}", web::delete().to(shipment_controller::delete_shipment))
            .route("/tracking/{tracking}", web::get().to(shipment_controller::get_by_tracking))
            .route("/tracking/{tracking}/status", web::patch().to(shipment_controller::update_status))
            .route("/tracking/{tracking}/proof", web::post().to(shipment_controller::upload_proof))
            .route("/status/{status}", web::get().to(shipment_controller::get_by_status))

    );

    cfg.service(
        web::scope("/payments")
            .route("", web::post().to(payment_controller::generate_payment))
            .route("", web::get().to(payment_controller::get_all_payments))
    
            // payment lookup
            .route("/reference/{ref}", web::get().to(payment_controller::get_payment_by_ref))
            .route("/shipment/{shipment_id}", web::get().to(payment_controller::get_payment_by_shipment_id))
    
            // status filtering
            .route("/status/{status}", web::get().to(payment_controller::get_payment_by_status))
    
            // revenue
            .route("/revenue/daily/{date}", web::get().to(payment_controller::get_daily_revenue))
            .route("/revenue/weekly/{date}", web::get().to(payment_controller::get_weekly_revenue))
            .route("/revenue/monthly/{month}/{year}", web::get().to(payment_controller::get_monthly_revenue))
    
            // webhook
            .route("/webhook", web::post().to(payment_controller::handle_webhook))
            
            // delete
            .route("/{ref}", web::delete().to(payment_controller::delete_payment))
    );
}