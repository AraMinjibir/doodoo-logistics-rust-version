
use crate::domain::models::address::Address;
use crate::domain::models::dimensions::Dimensions;
use crate::domain::models::proof_of_delivery::ProofOfDelivery;
use crate::domain::models::recipient::Recipient;
use crate::domain::models::shipment::Shipment;
use crate::domain::models::package_details:: PackageDetails;


pub fn test_shipment() -> Shipment {
    Shipment::create(
        "Ara".to_string(),
        test_recipient(),
        test_package(),
        None,
    )
    
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