use mockall::mock;
use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;


use crate::domain::models::{shipment::Shipment, payment::Payment};
use crate::domain::errors::{repository_error::RepositoryError, domain_error::DomainError};
use crate::repositories::{shipment_repository::ShipmentRepository, 
    payment_repository::PaymentRepository};
use crate::domain::gateways::{payment_gateway::PaymentGateway,
    payment_gateway::PaymentWebhookEvent, payment_gateway::PaymentGatewayResponse};

mock! {
    pub ShipmentRepo {}

    #[async_trait]
    impl ShipmentRepository for ShipmentRepo {
        async fn create(&self, shipment: &Shipment) -> Result<(), RepositoryError>;
        async fn update(&self, shipment: &Shipment) -> Result<(), RepositoryError>;
        async fn delete(&self, id: Uuid) -> Result<u64, RepositoryError>;

        async fn get_by_id(&self, id: Uuid) -> Result<Option<Shipment>, RepositoryError>;
        async fn get_by_status(&self, status: &str) -> Result<Vec<Shipment>, RepositoryError>;
        async fn find_by_tracking_number(&self, tracking: &str) -> Result<Option<Shipment>, RepositoryError>;
        async fn list_all(&self, offset: i64, limit: i64) -> Result<Vec<Shipment>, RepositoryError>;
        async fn assign_service_provider(&self, shipment_id: Uuid, provider_id: Uuid) -> Result<(), RepositoryError>;
        async fn upload_proof_of_delivery(&self, shipment_id: Uuid, proof: serde_json::Value) -> Result<Option<Shipment>, RepositoryError>;
        async fn find_by_service_provider(&self, provider_id: Uuid) -> Result<Vec<Shipment>, RepositoryError>;
    }

}

mock! {
    pub PaymentRepo {}

    #[async_trait]
    impl PaymentRepository for PaymentRepo{

        async fn persist_payment( &self, payment:&Payment) -> Result<(), RepositoryError>;

        async fn get_payment_by_ref(&self, reference:&str)-> Result<Option<Payment>, RepositoryError>;
        async fn get_payment_by_status(&self, status:&str) -> Result<Vec<Payment>, RepositoryError>;
        async fn get_payment_by_shipment_id(&self, shipment_id:Uuid) -> Result<Option<Payment>, RepositoryError>;
        async fn get_all_payments(&self)-> Result<Vec<Payment>, RepositoryError>;

        async fn update_payment(&self, payment:&Payment) -> Result<(), RepositoryError>;
        async fn delete_payment(&self, id:&str) -> Result<(), RepositoryError>;

        async fn get_daily_revenue(&self, date:NaiveDate) -> Result<Option<Decimal>, RepositoryError>;
        async fn get_weekly_revenue(&self, date:NaiveDate) -> Result<Option<Decimal>, RepositoryError>;
        async fn get_monthly_revenue(&self, year:u32, month: u32) -> Result<Option<Decimal>, RepositoryError>;

    }
    
}
mock!{
    pub Payment{}

    #[async_trait]
    impl PaymentGateway for Payment {
        
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
}
