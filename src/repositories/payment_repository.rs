use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{errors::repository_error::RepositoryError, 
    models::{payment::Payment, payment_status::PaymentStatus}};


#[async_trait::async_trait]
pub trait PaymentRepository: Send + Sync {

    async fn persist_payment( &self, payment:&Payment) -> Result<(), RepositoryError>;

    async fn get_payment_by_ref(&self, reference:&str)-> Result<Option<Payment>, RepositoryError>;
    async fn get_payment_by_status(&self, status:PaymentStatus) -> Result<Vec<Payment>, RepositoryError>;
    async fn get_payment_by_shipment_id(&self, shipment_id:Uuid) -> Result<Option<Payment>, RepositoryError>;
    async fn get_all_payments(&self)-> Result<Vec<Payment>, RepositoryError>;

    async fn update_payment(&self, payment:&Payment) -> Result<(), RepositoryError>;
    async fn delete_payment(&self, id:&str) -> Result<(), RepositoryError>;

    async fn get_daily_revenue(&self, date:NaiveDate) -> Result<Option<Decimal>, RepositoryError>;
    async fn get_weekly_revenue(&self, date:NaiveDate) -> Result<Option<Decimal>, RepositoryError>;
    async fn get_monthly_revenue(&self, year:u32, month: u32) -> Result<Option<Decimal>, RepositoryError>;

    
}