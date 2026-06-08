use uuid::Uuid;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::domain::models::payment::{PaymentMethod, PaymentCommand};

#[derive(Debug, Deserialize)]
pub struct GeneratePaymentDto {
    customer_id: Uuid,
    shipment_id: Uuid,
    amount: Decimal, 
    payment_method: PaymentMethod,
    
}

impl GeneratePaymentDto {
    pub fn to_domain(self) -> PaymentCommand {
    
        PaymentCommand::new(
            self.customer_id,
            self.shipment_id, 
            self.amount, 
            self.payment_method
        )
    }
    
}