use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;

use crate::{domain::{errors::domain_error::DomainError, 
    gateways::payment_gateway::PaymentGateway, 
    models::{payment::Payment, payment_status::PaymentStatus},
     services::payment_service::PaymentService}, 
      repositories::{payment_repository::PaymentRepository, 
     shipment_repository::ShipmentRepository} };

     

struct PaymentServiceImpl<R, S, G>
    where R:PaymentRepository {
        repo:R,
        shipment_repo: S,
        gateway: G,
}

impl <R, S, G>  PaymentServiceImpl<R, S, G>
where R: PaymentRepository,
        S: ShipmentRepository,
        G: PaymentGateway {

    pub fn new(repo:R, shipment_repo: S, gateway: G) -> Self{
        Self { repo, shipment_repo, gateway }
    }
    
}

#[async_trait::async_trait]
impl<R, S, G > PaymentService for PaymentServiceImpl<R, S, G >
where
    R: PaymentRepository,
    S: ShipmentRepository,
    G: PaymentGateway, {

    async fn generate_payment(
        &self, 
        callback_url:String, 
        payment: Payment
    ) -> Result<Payment, DomainError> {
            
            // Ensure shipment exists first
            self.shipment_repo.get_by_id(payment.shipment_id()).await?
            .ok_or(DomainError::ShipmentNotFoundById {
                id: payment.shipment_id(),
            })?;
            // prevent duplicate payment
            if self.repo
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
            let gateway_response = self.gateway
            .initiate_payment(&payment, &callback_url)
            .await?;

        // 5. Prepare payment to save
        let payment_to_save = payment.attach_gateway_response(
            gateway_response.reference
        );

        // 5. Save payment
        self.repo.persist_payment(&payment_to_save).await?;

        Ok(payment_to_save)


     }

     async fn get_payment_by_ref(&self, reference:&str)-> Result<Option<Payment>, DomainError> {
         let fetched_payment = self.repo.get_payment_by_ref(reference).await?;
         
        Ok(fetched_payment)
        }

    async fn get_payment_by_status(&self, status:&str) -> Result<Vec<Payment>, DomainError> {
        self.repo.get_payment_by_status(status).await
        .map_err(|e|DomainError::from(e))
    }

    async fn get_payment_by_shipment_id(&self, shipment_id:Uuid) -> Result<Option<Payment>, DomainError> {
       let paid_shipment =  self.repo.get_payment_by_shipment_id(shipment_id).await?;

       Ok(paid_shipment)
    }

    async fn get_all_payments(&self)-> Result<Vec<Payment>, DomainError> {
        self.repo.get_all_payments().await
        .map_err(|e| DomainError::from(e))

    }

    async fn get_daily_revenue(&self, date:NaiveDate) -> Result<Option<Decimal>, DomainError>{
        let daily_revenue = self.repo.get_daily_revenue(date).await?;

        Ok(daily_revenue)
        
    }
    async fn get_weekly_revenue(&self, date:NaiveDate) -> Result<Option<Decimal>, DomainError> {
       let weekly_revenue =  self.repo.get_weekly_revenue(date).await?;

       Ok(weekly_revenue)
    }
    async fn get_monthly_revenue(&self, year:u32, month: u32) -> Result<Option<Decimal>, DomainError> {
       let yearly =  self.repo.get_monthly_revenue(year, month).await?;

       Ok(yearly)
        
    }

    async fn handle_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<Payment, DomainError> {
    
        // 1. verify webhook
        let event = self.gateway
        .verify_webhook(payload, signature)
        .await?;
    
        // 2. fetch payment
        let payment = self.repo
            .get_payment_by_ref(&event.reference)
            .await
            .map_err(DomainError::from)?
            .ok_or(DomainError::PaymentNotFound {
                reference: event.reference.clone(),
            })?;
    
        // 3. determine status
        let new_status = if event.status == "success" {
            PaymentStatus::Successful
        } else {
            PaymentStatus::Failed
        };
    
        // 4. validate transition
        PaymentStatus::validate_transition(
            &payment.status(),
            &new_status,
        )?;
    
        // 5. update payment
        let updated_payment = payment.update_status(new_status);
    
        // 6. persist
        self.repo
            .update_payment(&updated_payment)
            .await
            .map_err(DomainError::from)?;
    
        Ok(updated_payment)
    }
    async fn delete_payment(&self, reference: &str) -> Result<(), DomainError> {

        // 1. Verify existence first
    let existing = self.repo
    .get_payment_by_ref(reference)
    .await
    .map_err(DomainError::from)?
    .ok_or(DomainError::PaymentNotFound {
        reference: reference.to_string(),
    })?;

    // 2. Perform delete
    self.repo
        .delete_payment(reference)
        .await
        .map_err(DomainError::from)?;

    Ok(())
    }
 }