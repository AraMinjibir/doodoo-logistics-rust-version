use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;

use crate::domain::{errors::domain_error::DomainError, models::payment::Payment};

#[async_trait]
pub trait PaymentService{

    async fn generate_payment( &self,callback_url:String, payment: &Payment) -> Result<Payment, DomainError>;

    async fn get_payment_by_ref(&self, reference:&str)-> Result<Option<Payment>, DomainError>;
    async fn get_payment_by_status(&self, status:&str) -> Result<Vec<Payment>, DomainError>;
    async fn get_payment_by_shipment_id(&self, shipment_id:Uuid) -> Result<Option<Payment>, DomainError>;
    async fn get_all_payments(&self)-> Result<Vec<Payment>, DomainError>;
    async fn handle_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<Payment, DomainError> ;
    async fn delete_payment(&self, id:&str) -> Result<(), DomainError>;

    async fn get_daily_revenue(&self, date:NaiveDate) -> Result<Option<Decimal>, DomainError>;
    async fn get_weekly_revenue(&self, date:NaiveDate) -> Result<Option<Decimal>, DomainError>;
    async fn get_monthly_revenue(&self, year:u32, month: u32) -> Result<Option<Decimal>, DomainError>;
}