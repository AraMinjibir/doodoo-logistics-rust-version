use actix_web::web;
use crate::controllers::shipment_controller;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
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
}