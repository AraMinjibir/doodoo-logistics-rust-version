use uuid::Uuid;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{domain::{errors::domain_error::DomainError, models::payment::{Payment, PaymentMethod}}};

#[derive(Debug, Deserialize)]
pub struct GeneratePaymentDto {
    customer_id: Uuid,
    shipment_id: Uuid,
    amount: Decimal, 
    payment_method: PaymentMethod,
    
}

impl GeneratePaymentDto {
    pub fn to_domain(self) -> Result<Payment, DomainError> {
    
    Payment::generate_payment(
        self.customer_id, 
        self.shipment_id, 
        self.amount, 
        self.payment_method
    )
    }
    
}