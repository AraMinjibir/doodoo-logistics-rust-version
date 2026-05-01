
use crate::domain::models::address::Address;
use crate::domain::models::dimensions::Dimensions;
use crate::domain::models::proof_of_delivery::ProofOfDelivery;
use crate::domain::models::recipient::Recipient;
use crate::domain::models::shipment::{Shipment, UpdateShipment};
use crate::domain::models::package_details:: PackageDetails;


pub fn test_shipment() -> Shipment {
    Shipment::create(
        "Ara".to_string(),
        test_recipient(),
        test_package(),
        None,
    )
    
}
pub fn updated_shipment() -> UpdateShipment {
    let address = Address::create(
        "123 Street".to_string(),
        "Kano".to_string(),
        "Kano".to_string(),
        "Nigeria".to_string(),
        "700001".to_string(),
    ).unwrap();

    let recipient = Recipient::create(
        "John Doe".to_string(),
        "08012345678".to_string(),
        address,
    ).unwrap();

    let dimensions = Dimensions::create(10.0, 5.0, 2.0).unwrap();

    let package_details = PackageDetails::create(
        2.5,
        dimensions,
        "Books".to_string(),
    ).unwrap();

    UpdateShipment {
        sender_name: Some("DooDOo".to_string()),
        recipient: Some(recipient),
        package_details: Some(package_details),
    }
}


fn test_package() -> PackageDetails{
    let dimensions = Dimensions::create(10.2, 12.2, 14.3).unwrap();
    PackageDetails::create(
        20.3,
        dimensions,
        "phones".to_string())
        .unwrap()
}

pub fn test_proof() -> ProofOfDelivery{
    ProofOfDelivery::create(
    Some("image".to_string()),
       "note".to_string(), 
       "submitted_by".to_string()
    ).unwrap()
}
fn test_recipient() -> Recipient{
    let addres = Address::create("street".to_string(),
     "city".to_string(),
     " state".to_string(),
      "country".to_string(),
       "postal_code".to_string()
    ).unwrap();

    Recipient::create("Minjibir".to_string(), 
   "0700200202020".to_string(),
    addres).unwrap()
}