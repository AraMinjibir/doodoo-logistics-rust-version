use crate::domain::errors::domain_error::DomainError;
use crate::domain::gateways::payment_gateway::{
    PaymentGateway, PaymentGatewayResponse, PaymentWebhookEvent,
};
use crate::domain::models::payment::Payment;
use async_trait::async_trait;
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
    ) -> Result<PaymentGatewayResponse, DomainError> {
        let fake_url = format!(
            "https://mock-payments.local/pay/{}",
            payment.reference_number()
        );

        Ok(PaymentGatewayResponse {
            authorization_url: fake_url,
            reference: payment.reference_number(),
        })
    }

    async fn verify_webhook(
        &self,
        _event: &PaymentWebhookEvent,
        _signature: &str,
    ) -> Result<(), DomainError> {
        Ok(())
    }
}
