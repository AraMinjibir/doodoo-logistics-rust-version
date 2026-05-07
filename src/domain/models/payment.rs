
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::domain::models::payment_status::PaymentStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentMethod { Card, MobileMoney, BankTransfer }


#[derive(Debug, Clone)]
pub struct Payment {
    customer_id: Uuid,
    shipment_id: Uuid,
    amount: Decimal, 
    status: PaymentStatus,
    paid_at: DateTime<Utc>,
    payment_method: PaymentMethod,
    reference_number: String,
    gateway_transaction_id: Option<String>,
    failure_reason: Option<String>,
}


impl Payment {
    pub fn generate_payment(
        customer_id: Uuid,
        shipment_id: Uuid,
        amount: Decimal,
        payment_method: PaymentMethod,
    ) -> Result<Self, Vec<String>> {
        let mut errors = Vec::new();

        // 1. Validation Logic (The "Sad Path")
        if customer_id.is_nil() {
            errors.push(format!("CustomerId must be provided: {}", customer_id));
        }
        if shipment_id.is_nil() {
            errors.push(format!("Shipment id must be provided: {}", shipment_id));
        }
        if amount <= Decimal::ZERO {
            errors.push(format!("Amount must be positive: {}", amount));
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        // 2. The "Happy Path"
        let now = Utc::now();
        Ok(Self {
            customer_id,
            shipment_id,
            amount,
            status: PaymentStatus::Pending,
            paid_at: now,
            payment_method,
            reference_number: Self::generate_reference_number(),
            gateway_transaction_id: None,
            failure_reason: None,
        })
    }

    fn generate_reference_number() -> String {
        let id = Uuid::new_v4().to_string().replace("-", "");
        format!("RF-DODO-{}", &id[..10].to_uppercase())
    }


    // Getters

    pub fn reference_number(&self) -> String {
        self.reference_number.clone()
    }
}