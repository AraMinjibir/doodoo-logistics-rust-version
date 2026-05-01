use std::sync::Arc;
use crate::domain::services::shipment_service::ShipmentService;

pub struct AppState {
    pub shipment_service: Arc<dyn ShipmentService + Send + Sync>,
}