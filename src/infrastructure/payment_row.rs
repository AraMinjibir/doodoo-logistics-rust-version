use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;


use crate::domain::models::payment_status::PaymentStatus;
use crate::domain::models::{
   payment::PaymentMethod,
   payment::Payment
};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PaymentRow {
   pub customer_id: Uuid,
   pub shipment_id: Uuid,
   pub amount: Decimal, 
   pub status: String,
   pub paid_at: DateTime<Utc>,
   pub payment_method: String,
   pub reference_number: String,
   pub gateway_transaction_id: Option<String>,
   pub failure_reason: Option<String>,
}

impl PaymentRow {

   // Convert DB Row to Domain Model
   pub fn into_domain(self) -> Payment {
       Payment::reconstitute(
         self.customer_id, 
        self.shipment_id, 
        self.amount, 
        PaymentStatus::from_string(&self.status)
        .expect("Invalid payment status in DB"), 
        self.paid_at,
       PaymentMethod::from_string(&self.payment_method)
            .expect("Invalid payment method in DB"), 

        self.reference_number, 
        self.gateway_transaction_id, 
        self.failure_reason
      )  
       
       }
   

   // Convert Domain Model to DB Row
   pub fn from_domain(payment: Payment) -> Self {
       Self {
           reference_number: payment.reference_number(),
           customer_id: payment.customer_id(),
           shipment_id: payment.shipment_id(),
           amount: payment.amount(),
           status: payment.status().to_string(),
           payment_method: payment.payment_method().to_string(),
           paid_at: payment.paid_at(),
           gateway_transaction_id: payment.gateway_transaction_id(),
           failure_reason: payment.failure_reason(),
       }
   }
}   
