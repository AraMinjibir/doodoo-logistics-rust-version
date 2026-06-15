use crate::domain::errors::domain_error::DomainError;
use crate::domain::models::payment::Payment;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn initiate_payment(
        &self,
        payment: &Payment,
    ) -> Result<PaymentGatewayResponse, DomainError>;

    async fn verify_webhook(
        &self,
        event: &PaymentWebhookEvent,
        signature: &str,
    ) -> Result<(), DomainError>;
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PaymentGatewayResponse {
    pub authorization_url: String,
    pub reference: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaymentWebhookEvent {
    pub reference: String,
    pub status: String,
    pub gateway_transaction_id: Option<String>,
}
