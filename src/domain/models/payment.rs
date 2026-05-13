
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use std::fmt;

use crate::domain::{errors::domain_error::DomainError, models::payment_status::PaymentStatus};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod { Card, MobileMoney, BankTransfer }

impl PaymentMethod {
    pub fn methods() -> &'static [PaymentMethod] {

        &[
            Self::Card,
            Self::MobileMoney,
            Self::BankTransfer,
            
        ]
     }

     pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "Card" => Some(Self::Card),
            "MobileMoney" => Some(Self::MobileMoney),
            "BankTransfer" => Some(Self::BankTransfer),
            _ => None
            
        }
     }
    
}
impl fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Card => "Card",
            Self::MobileMoney => "MobileMoney",
            Self::BankTransfer => "BankTransfer",
        };
        write!(f, "{}", value)
    }
}


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

#[allow(dead_code)]
impl Payment {
    pub fn generate_payment(
        customer_id: Uuid,
        shipment_id: Uuid,
        amount: Decimal,
        payment_method: PaymentMethod,
    ) -> Result<Self, DomainError> {
        let mut errors = Vec::new();

        // 1. Validation Logic (The "Sad Path")
        if customer_id.is_nil() {
          errors.push("Customer id must not be empty".to_string());
        }
        
        if shipment_id.is_nil() {
          errors.push("Shipment id  must not be empty".to_string());
        }
        if amount <= Decimal::ZERO {
            errors.push(format!("Amount must be a positive value: {}", amount));
        }
        if !errors.is_empty() {
            return Err(DomainError::ValidationError(errors));
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

    pub fn attach_gateway_response(
        mut self,
        reference_number: String,
    ) -> Self {
        self.reference_number = reference_number;
        self.status = PaymentStatus::Pending;
        self
    }

    pub fn update_status(
        mut self,
        status: PaymentStatus,
    ) -> Self {

        self.status = status;
        self
    }
    // Getters and Setters

    pub fn reference_number(&self) -> String {
        self.reference_number.clone()
    }
    pub fn customer_id(&self) -> Uuid{
        self.customer_id
    }
    pub fn shipment_id(&self) -> Uuid{
        self.shipment_id
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn status(&self) -> PaymentStatus{
        self.status.clone()
    }

    pub fn payment_method(&self) -> PaymentMethod {
        self.payment_method.clone()
    }

    pub fn paid_at(&self) -> DateTime<Utc> {
        self.paid_at
    }

    pub fn   gateway_transaction_id(&self) -> Option<String> {
        self.gateway_transaction_id.clone()
    }

    pub fn failure_reason(&self) -> Option<String>{
        self.failure_reason.clone()
    }

    pub fn set_status(&mut self,status:PaymentStatus){
        self.status = status;
    }

    pub fn set_failure_reason(&mut self, reason: Option<String>) {
        self.failure_reason = reason;
    }
    
    pub fn set_gateway_transaction_id(&mut self, id: Option<String>) {
        self.gateway_transaction_id = id;
    }

    pub fn set_paid_at(&mut self, paid: DateTime<Utc>){
        self.paid_at = paid;
    }
    pub fn reconstitute(
        customer_id: Uuid,
    shipment_id: Uuid,
    amount: Decimal, 
    status: PaymentStatus,
    paid_at: DateTime<Utc>,
    payment_method: PaymentMethod,
    reference_number: String,
    gateway_transaction_id: Option<String>,
    failure_reason: Option<String>
    ) -> Self {
        Self { 
            customer_id,
            shipment_id,
            amount, status,
            paid_at, payment_method, 
            reference_number, 
            gateway_transaction_id, 
            failure_reason }
    }
}