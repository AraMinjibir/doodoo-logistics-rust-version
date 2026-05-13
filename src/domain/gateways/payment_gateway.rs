use async_trait::async_trait;
use crate::domain::models::payment::Payment;
use crate::domain::errors::domain_error::DomainError;

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn initiate_payment(
        &self,
        payment: &Payment,
        callback_url: &str,
    ) -> Result<PaymentGatewayResponse, DomainError>;

    async fn verify_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<PaymentWebhookEvent, DomainError>;
}

#[derive(Debug, Clone)]
pub struct PaymentGatewayResponse {
    pub authorization_url: String,
    pub reference: String,
}

#[derive(Debug, Clone)]
pub struct PaymentWebhookEvent {
    pub reference: String,
    pub status: String,
    pub gateway_transaction_id: Option<String>,
}