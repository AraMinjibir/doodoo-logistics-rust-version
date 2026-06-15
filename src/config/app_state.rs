use crate::domain::services::{payment_service::PaymentService, shipment_service::ShipmentService};
use std::sync::Arc;

pub struct AppState {
    pub shipment_service: Arc<dyn ShipmentService + Send + Sync>,
    pub payment_service: Arc<dyn PaymentService + Send + Sync>,
}
