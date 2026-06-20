use chrono::{DateTime, Utc};

use crate::domain::models::address::Address;
use crate::domain::models::dimensions::Dimensions;
use crate::domain::models::package_details::PackageDetails;
use crate::domain::models::proof_of_delivery::ProofOfDelivery;
use crate::domain::models::recipient::Recipient;
use crate::domain::models::shipment::Shipment;
use crate::infrastructure::shipment_row::ShipmentRow;

pub struct ShipmentMapper;

impl ShipmentMapper {
    fn to_utc(ndt: chrono::NaiveDateTime) -> DateTime<Utc> {
        DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc)
    }

    fn deserialize_pod(value: serde_json::Value) -> Vec<ProofOfDelivery> {
        serde_json::from_value(value).unwrap_or_else(|_| vec![])
    }

    pub fn from_row(row: ShipmentRow) -> Shipment {
        Shipment::reconstitute(
            row.id,
            row.sender_name.clone(),
            RecipientMapper::from_row(&row),
            PackageDetailsMapper::from_row(&row),
            row.tracking_number.expect("tracking_number missing in DB"),
            row.status.parse().expect("Invalid shipment status in DB"),
            Self::to_utc(row.created_at),
            Self::to_utc(row.updated_at),
            Self::deserialize_pod(row.proof_of_delivery),
            row.service_provider_id,
        )
    }

    pub fn to_row(shipment: Shipment) -> ShipmentRow {
        ShipmentRow {
            id: shipment.id(),
            tracking_number: Some(shipment.tracking_number().to_string()),
            sender_name: shipment.sender_name().to_string(),

            recipient_name: shipment.recipient().name().to_string(),
            recipient_street: shipment.recipient().address().street().to_string(),
            recipient_city: shipment.recipient().address().city().to_string(),
            recipient_state: shipment.recipient().address().state().to_string(),
            recipient_country: shipment.recipient().address().country().to_string(),
            recipient_postal_code: shipment.recipient().address().postal_code().to_string(),
            recipient_contact: shipment.recipient().contact().to_string(),

            weight: shipment.package_details().weight(),
            length: shipment.package_details().dimensions().length(),
            width: shipment.package_details().dimensions().width(),
            height: shipment.package_details().dimensions().height(),
            contents: shipment.package_details().contents().to_string(),

            status: shipment.status().to_string(),
            estimated_delivery_date: Some(shipment.delivery_date_estimate().naive_utc()),
            created_at: shipment.created_at().naive_utc(),
            updated_at: shipment.updated_at().naive_utc(),

            cost: shipment.cost(),
            proof_of_delivery: serde_json::json!(shipment.proof_of_delivery()),
            service_provider_id: shipment.service_provider_id(),
        }
    }
}

pub struct RecipientMapper;

impl RecipientMapper {
    pub fn from_row(row: &ShipmentRow) -> Recipient {
        let address = Address::create(
            row.recipient_street.clone(),
            row.recipient_city.clone(),
            row.recipient_state.clone(),
            row.recipient_country.clone(),
            row.recipient_postal_code.clone(),
        )
        .expect("Invalid address in DB");

        Recipient::create(
            row.recipient_name.clone(),
            row.recipient_contact.clone(),
            address,
        )
        .expect("Invalid recipient in DB")
    }
}

pub struct PackageDetailsMapper;

impl PackageDetailsMapper {
    pub fn from_row(row: &ShipmentRow) -> PackageDetails {
        let dimensions = Dimensions::create(row.length, row.width, row.height)
            .expect("Invalid dimensions in DB");

        PackageDetails::create(row.weight, dimensions, row.contents.clone())
            .expect("Invalid package details in DB")
    }
}
