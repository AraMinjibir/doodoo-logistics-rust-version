use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::models::payment::{PaymentCommand, PaymentMethod};

#[derive(Debug, Deserialize)]
pub struct GeneratePaymentDto {
    customer_id: Uuid,
    shipment_id: Uuid,
    amount: Decimal,
    payment_method: PaymentMethod,
}

impl GeneratePaymentDto {
    pub fn into_domain(self) -> PaymentCommand {
        PaymentCommand::new(
            self.customer_id,
            self.shipment_id,
            self.amount,
            self.payment_method,
        )
    }
}
