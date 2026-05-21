use std::sync::Arc;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    domain::{
        errors::domain_error::DomainError,
        gateways::{payment_gateway::PaymentGateway, payment_gateway::PaymentWebhookEvent},
        models::{payment::Payment, payment_status::PaymentStatus},
        services::payment_service::PaymentService,
    },
    repositories::{
        payment_repository::PaymentRepository, shipment_repository::ShipmentRepository,
    },
};

pub struct PaymentServiceImpl {
    payment_repo: Arc<dyn PaymentRepository + Send + Sync>,
    shipment_repo: Arc<dyn ShipmentRepository + Send + Sync>,
    gateway: Arc<dyn PaymentGateway + Send + Sync>,
}

impl PaymentServiceImpl {
    pub fn new(
        payment_repo: Arc<dyn PaymentRepository + Send + Sync>,
        shipment_repo: Arc<dyn ShipmentRepository + Send + Sync>,
        gateway: Arc<dyn PaymentGateway + Send + Sync>,
    ) -> Self {
        Self {
            payment_repo,
            shipment_repo,
            gateway,
        }
    }
}

#[async_trait::async_trait]
impl PaymentService for PaymentServiceImpl {
    async fn generate_payment(&self, payment: &Payment) -> Result<Payment, DomainError> {
        // Ensure shipment exists first
        self.shipment_repo
            .get_by_id(payment.shipment_id())
            .await?
            .ok_or(DomainError::ShipmentNotFoundById {
                id: payment.shipment_id(),
            })?;
        // prevent duplicate payment
        if self
            .payment_repo
            .get_payment_by_shipment_id(payment.shipment_id())
            .await?
            .is_some()
        {
            return Err(DomainError::PaymentExistsForThisShipment {
                id: payment.shipment_id(),
            });
        }
        // 3. Create payment
        let payment = Payment::generate_payment(
            payment.customer_id(),
            payment.shipment_id(),
            payment.amount(),
            payment.payment_method(),
        )?;

        // 4. Call gateway
        let gateway_response = self.gateway.initiate_payment(&payment).await?;

        // 5. Prepare payment to save
        let payment_to_save = payment.attach_gateway_response(gateway_response.reference);

        // 5. Save payment
        self.payment_repo.persist_payment(&payment_to_save).await?;

        Ok(payment_to_save)
    }

    async fn get_payment_by_ref(&self, reference: &str) -> Result<Payment, DomainError> {
        let fetched_payment = self.payment_repo.get_payment_by_ref(reference).await?;

        match fetched_payment {
            Some(payment) => Ok(payment),
            None => Err(DomainError::PaymentNotFound {
                reference: reference.to_string(),
            }),
        }
    }

    async fn get_payment_by_status(&self, status: &str) -> Result<Vec<Payment>, DomainError> {
        self.payment_repo
            .get_payment_by_status(status)
            .await
            .map_err(|e| DomainError::from(e))
    }

    async fn get_payment_by_shipment_id(&self, shipment_id: Uuid) -> Result<Payment, DomainError> {
        let paid_shipment = self
            .payment_repo
            .get_payment_by_shipment_id(shipment_id)
            .await?;

        match paid_shipment {
            Some(paid) => Ok(paid),
            None => Err(DomainError::PaymentWithShipmentIdNotFound { shipment_id }),
        }
    }

    async fn get_all_payments(&self) -> Result<Vec<Payment>, DomainError> {
        self.payment_repo
            .get_all_payments()
            .await
            .map_err(|e| DomainError::from(e))
    }

    async fn get_daily_revenue(&self, date: NaiveDate) -> Result<Decimal, DomainError> {
        let daily_revenue = self.payment_repo.get_daily_revenue(date).await?;

        match daily_revenue {
            Some(revenue) => Ok(revenue),
            None => Err(DomainError::RevenueNotFoundWithDate { date }),
        }
    }
    async fn get_weekly_revenue(&self, date: NaiveDate) -> Result<Decimal, DomainError> {
        let weekly_revenue = self.payment_repo.get_weekly_revenue(date).await?;

        match weekly_revenue {
            Some(revenue) => Ok(revenue),
            None => Err(DomainError::RevenueNotFoundWithDate { date }),
        }
    }
    async fn get_monthly_revenue(&self, year: u32, month: u32) -> Result<Decimal, DomainError> {
        let yearly = self.payment_repo.get_monthly_revenue(year, month).await?;

        match yearly {
            Some(revenue) => Ok(revenue),
            None => Err(DomainError::RevenueNotFound { month }),
        }
    }

    async fn handle_webhook(
        &self,
        event: &PaymentWebhookEvent,
        signature: &str,
    ) -> Result<Payment, DomainError> {
        // 1. verify webhook authenticity
        self.gateway.verify_webhook(event, signature).await?;

        // 2. fetch payment
        let payment = self
            .payment_repo
            .get_payment_by_ref(&event.reference)
            .await
            .map_err(DomainError::from)?
            .ok_or(DomainError::PaymentNotFound {
                reference: event.reference.clone(),
            })?;

        // 3. derive status
        let new_status = if event.status == "success" {
            PaymentStatus::Successful
        } else {
            PaymentStatus::Failed
        };

        // 4. validate invariant
        if matches!(new_status, PaymentStatus::Successful) && event.gateway_transaction_id.is_none()
        {
            return Err(DomainError::PaymentGatewayError {
                signature: "missing transaction id".into(),
            });
        }

        // 5. idempotency
        if payment.status() == new_status
            && payment.gateway_transaction_id() == event.gateway_transaction_id
        {
            return Ok(payment);
        }

        // 6. validate transition
        PaymentStatus::validate_transition(&payment.status(), &new_status)?;

        // 7. update
        let updated_payment = payment.update_status(
            new_status,
            event.gateway_transaction_id.clone(),
        )?;

        // 8. persist
        self.payment_repo
            .update_payment(&updated_payment)
            .await
            .map_err(DomainError::from)?;

        Ok(updated_payment)
    }

    async fn delete_payment(&self, reference: &str) -> Result<(), DomainError> {
        // 1. Verify existence first
        self.payment_repo
            .get_payment_by_ref(reference)
            .await
            .map_err(DomainError::from)?
            .ok_or(DomainError::PaymentNotFound {
                reference: reference.to_string(),
            })?;

        // 2. Perform delete
        self.payment_repo
            .delete_payment(reference)
            .await
            .map_err(DomainError::from)?;

        Ok(())
    }
}
