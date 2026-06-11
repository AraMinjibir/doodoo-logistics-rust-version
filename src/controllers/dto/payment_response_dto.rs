use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::{
    payment::{GeneratePaymentResponse, Payment, PaymentMethod},
    payment_status::PaymentStatus,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResponseDto {
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

impl PaymentResponseDto {
    pub fn from_domain(payment: Payment) -> Self {
        Self {
            reference_number: payment.reference_number(),
            customer_id: payment.customer_id(),
            shipment_id: payment.shipment_id(),
            amount: payment.amount(),
            status: payment.status(),
            paid_at: payment.paid_at(),
            payment_method: payment.payment_method(),
            gateway_transaction_id: payment.gateway_transaction_id(),
            failure_reason: payment.failure_reason(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratePaymentResponseDto {
    pub reference_number: String,
    pub status: PaymentStatus,
    pub authorization_url: String,
}

impl GeneratePaymentResponseDto {
    pub fn from_response(response: GeneratePaymentResponse) -> Self {
        Self {
            reference_number: response.payment.reference_number(),
            status: response.payment.status(),
            authorization_url: response.authorization_url,
        }
    }
}
