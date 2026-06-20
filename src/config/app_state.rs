use crate::domain::services::{
    payment_service::PaymentService, shipment_service::ShipmentService,
    support_service::SupportService,
};
use std::sync::Arc;

pub struct AppState {
    pub shipment_service: Arc<dyn ShipmentService + Send + Sync>,
    pub payment_service: Arc<dyn PaymentService + Send + Sync>,
    pub support_service: Arc<dyn SupportService + Send + Sync>,
}
