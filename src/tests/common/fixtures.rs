#![allow(dead_code)] // This tells Rust to only look at this folder during 'cargo test'

use serde_json::{json, Value};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

use crate::domain::models::{
    address::Address, dimensions::Dimensions,
    payment::Payment,payment::PaymentCommand, proof_of_delivery::ProofOfDelivery,
    recipient::Recipient,shipment::{Shipment, UpdateShipment},
    package_details:: PackageDetails,payment::PaymentMethod,
    payment_status::PaymentStatus
};


#[allow(dead_code)]

pub fn test_shipment() -> Shipment {
    let service_provider_id =  Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();
    Shipment::create(
        "Ara".to_string(),
        test_recipient(),
        test_package(),
        Some(service_provider_id),    
    ).expect("Test shipment should be valid")
    
}

pub fn test_payment(shipment_id:Uuid) -> Payment {
    let customer_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    Payment::generate_payment(
        customer_id,
        shipment_id,
        Decimal::new(1000, 0),
        PaymentMethod::Card,
    )
    .expect("Test payment should be valid")

}
pub fn test_command(shipment_id: Uuid) -> PaymentCommand {
    let customer_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();

    PaymentCommand {
        customer_id,
        shipment_id,
        amount: Decimal::new(1000, 0),
        payment_method: PaymentMethod::Card,
    }
}


pub fn test_success_payment(
    shipment_id: Uuid,
    amount: Decimal,
    paid_at: DateTime<Utc>,
) -> Payment {
    let customer_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();

    let mut payment = Payment::generate_payment(
        customer_id,
        shipment_id,
        amount,
        PaymentMethod::Card,
    )
    .expect("Test payment should be valid");

    payment.set_status(PaymentStatus::Successful); 
    payment.set_paid_at(paid_at);

    payment
}

pub fn create_shipment_payload() -> Value {
    json!({
        "sender_name": "Ara",
            "recipient_name": "DooDoo",
            "street": "Zoo Road",
            "city": "Kano",
            "state": "Kano",
            "country": "Nigeria",
            "postal_code": "700001",
            "contact": "+2347012345678",
            "weight": 2.5,
            "length": 10.0,
            "width": 5.0,
            "height": 3.0,
            "contents": "Books",
            "service_provider_id": "22222222-2222-2222-2222-222222222222"
    })
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



