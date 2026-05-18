use async_trait::async_trait;
use serde_json::Value;
use crate::domain::models::payment::Payment;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::gateways:: payment_gateway::{
   PaymentGateway,
    PaymentGatewayResponse,
    PaymentWebhookEvent
};
pub struct MockPaymentGateway;

impl MockPaymentGateway {

    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PaymentGateway for MockPaymentGateway {
    async fn initiate_payment(
        &self,
        payment: &Payment,
        _callback_url: &str, 
    ) -> Result<PaymentGatewayResponse, DomainError> {
        
        let fake_url = format!("https://mock-payments.local/pay/{}", payment.reference_number());

        // Equivalent to Future.successful
        Ok(PaymentGatewayResponse {
            authorization_url: fake_url,
            reference: payment.reference_number(),
        })
    }

  async  fn verify_webhook(
        &self,
        payload: &str,
        _signature: &str,
    ) -> Result<PaymentWebhookEvent, DomainError> {
        // Parse JSON using Serde
        let json: Value = serde_json::from_str(payload)
            .map_err(|_| DomainError::Internal("Invalid JSON payload".to_string()))?;

        // Extract reference from data.reference
        let reference = json["data"]["reference"]
            .as_str()
            .ok_or_else(|| DomainError::Internal("Reference not found in payload".to_string()))?;

        Ok(PaymentWebhookEvent {
            reference: reference.to_string(),
            status: "success".to_string(),
            gateway_transaction_id: Some("MOCK_TX_123".to_string()),
        })
    }
}