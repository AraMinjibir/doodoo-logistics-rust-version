use serde::{Deserialize, Serialize};

use crate::domain::models::{
    shipment::Shipment,
    recipient::Recipient,
    address::Address,
    package_details::PackageDetails,
    dimensions::Dimensions,
    proof_of_delivery::ProofOfDelivery,
    shipment::UpdateShipment,
};
use crate::domain::errors::domain_error::DomainError;
use crate::controllers::helpers::shipment_helper::IntoDomain;


#[derive(Debug, Deserialize)]
pub(crate) struct CreateShipmentDto {
    pub sender_name: String,
    pub recipient_name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
    pub contact: String,
    pub weight: f64,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub contents: String,
}

#[allow(dead_code)]
impl CreateShipmentDto {
    pub fn to_domain(self) -> Result<Shipment, DomainError> {
        let address = Address::create(
            self.street,
            self.city,
            self.state,
            self.country,
            self.postal_code,
        )
        .map_err(|errs| DomainError::ValidationError(errs))?;

        let dimensions = Dimensions::create(
            self.length,
            self.width,
            self.height,
        )
        .map_err(|errs| DomainError::ValidationError(errs))?;

        let package_details = PackageDetails::create(
            self.weight,
            dimensions,
            self.contents,
        )
        .map_err(|errs| DomainError::ValidationError(errs))?;

        let recipient = Recipient::create(
            self.recipient_name.clone(),
            self.contact,
            address,
        )
        .map_err(|errs| DomainError::ValidationError(errs))?;

        Ok(Shipment::create(
            self.sender_name,
            recipient,
            package_details,
            None,
        )?)
    }
}
#[derive(Debug, Deserialize)]
pub struct UpdateShipmentDto {
    pub sender_name: Option<String>,
    pub recipient_name: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub contact: Option<String>,
    pub weight: Option<f64>,
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub contents: Option<String>,
}
impl UpdateShipmentDto {
    pub fn into_command(self) -> Result<UpdateShipment, DomainError> {
        let dimensions = match (self.length, self.width, self.height) {
            (Some(l), Some(w), Some(h)) => {
                match Dimensions::create(l, w, h) {
                    Ok(dim) => Some(dim),
                    Err(errs) => {
                        return Err(DomainError::ValidationError(
                            errs.into_iter().map(|e| e.to_string()).collect()
                        ));
                    }
                }
            }
        
            (None, None, None) => None,
        
            _ => {
                return Err(DomainError::ValidationError(vec![
                    "Incomplete dimensions provided".to_string()
                ]));
            }
        };
        let package_details = match (self.weight, self.contents, dimensions) {
            (Some(weight), Some(contents), Some(dim)) => {
                match PackageDetails::create(weight, dim, contents) {
                    Ok(pd) => Some(pd),
                    Err(errs) => {
                        return Err(DomainError::ValidationError(
                            errs.into_iter().map(|e| e.to_string()).collect()
                        ));
                    }
                }
            }
        
            (None, None, None) => None,
        
            _ => {
                return Err(DomainError::ValidationError(vec![
                    "Incomplete package details provided".to_string()
                ]));
            }
        };
        let recipient = match (
            self.recipient_name,
            self.contact,
            self.street,
            self.city,
            self.state,
            self.country,
            self.postal_code,
        ) {
            (
                Some(name),
                Some(contact),
                Some(street),
                Some(city),
                Some(state),
                Some(country),
                Some(postal),
            ) => {
                let address = Address::create(street, city, state, country, postal)
                    .map_err(DomainError::ValidationError)?;
        
                Some(
                    Recipient::create(name, contact, address)
                        .map_err(DomainError::ValidationError)?
                )
            }
            _ => None,
        };

        Ok(UpdateShipment {
            sender_name: self.sender_name,
            recipient, 
            package_details,
        })
    }
}

#[derive(Deserialize)]
pub(crate) struct PaginationQuery {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ProofOfDeliveryDto {
    pub image: Option<String>,
    pub note: String,
    pub submitted_by: String,
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl ProofOfDeliveryDto {
    pub fn to_domain(self) -> Result<ProofOfDelivery, DomainError> {
        ProofOfDelivery::create(
            self.image,
            self.note,
            self.submitted_by,
        )
        .map_err(|errs| {
            DomainError::ValidationError(
                errs.into_iter().map(|e| e.to_string()).collect()
            )
        })
    }
}

#[derive(Debug, Serialize)]
pub struct DimensionsDto {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}
impl From<Dimensions> for DimensionsDto {
    fn from(d: Dimensions) -> Self {
        Self {
            length: d.length(),
            width: d.width(),
            height: d.height(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RecipientDto {
    pub name: String,
    pub contact: String,
    pub address: AddressDto
}

#[derive(Debug, Serialize)]
pub struct PackageDetailsDto {
    pub dimensions: DimensionsDto,
    pub weight: String,
    pub contents: String,
}

#[derive(Debug, Serialize)]
pub struct AddressDto {
    pub country: String,
    pub city: String,
    pub state: String,
}


impl IntoDomain<Shipment> for CreateShipmentDto {
    fn to_domain(self) -> Result<Shipment, DomainError> {
        let address = Address::create(
            self.street,
            self.city,
            self.state,
            self.country,
            self.postal_code,
        )
        .map_err(DomainError::ValidationError)?;

        let dimensions = Dimensions::create(
            self.length,
            self.width,
            self.height,
        )
        .map_err(DomainError::ValidationError)?;

        let package_details = PackageDetails::create(
            self.weight,
            dimensions,
            self.contents,
        )
        .map_err(DomainError::ValidationError)?;

        let recipient = Recipient::create(
            self.recipient_name.clone(),
            self.contact,
            address,
        )
        .map_err(DomainError::ValidationError)?;

        Ok(Shipment::create(
            self.sender_name,
            recipient,
            package_details,
            None,
        )?)
    }
}

#[derive(Deserialize)]
pub struct UpdateStatusDto {
    pub status: String,
}




