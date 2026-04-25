use crate::domain::models::shipment::Shipment;
use crate::infrastructure::shipment_row::ShipmentRow;
use crate::domain::errors::repository_error::RepositoryError;

pub struct ShipmentMapper;

impl ShipmentMapper {
    pub fn to_domain(row: ShipmentRow) -> Shipment {
        Shipment::from(row)
    }

    pub fn to_domain_list(rows: Vec<ShipmentRow>) -> Vec<Shipment> {
        rows.into_iter().map(Self::to_domain).collect()
    }

    pub fn from_domain(shipment: Shipment) -> ShipmentRow {
        shipment.into()
    }

    
}
pub fn map_sqlx_error(_: sqlx::Error) -> RepositoryError {
    RepositoryError::DatabaseError
}