use std::sync::Arc;
use crate::domain::services::{payment_service::PaymentService, shipment_service::ShipmentService};

pub struct AppState {
    pub shipment_service: Arc<dyn ShipmentService + Send + Sync>,
    pub payment_service: Arc<dyn PaymentService + Send + Sync>,
}