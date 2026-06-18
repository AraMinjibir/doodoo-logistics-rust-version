use async_trait::async_trait;
use chrono::NaiveDate;
use mockall::mock;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::errors::{domain_error::DomainError, repository_error::RepositoryError};
use crate::domain::gateways::{
    payment_gateway::PaymentGateway, payment_gateway::PaymentGatewayResponse,
    payment_gateway::PaymentWebhookEvent,
};
use crate::domain::models::{payment::Payment, shipment::Shipment, support::Complaint, support_status::SupportStatus};
use crate::repositories::{
    payment_repository::PaymentRepository, shipment_repository::ShipmentRepository,support_repository::SupportRepository,
};

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
mock! {
    pub Payment{}

    #[async_trait]
    impl PaymentGateway for Payment {

        async fn initiate_payment(
            &self,
            payment: &Payment
        ) -> Result<PaymentGatewayResponse, DomainError>;

        async fn verify_webhook(
            &self,
            payload:&PaymentWebhookEvent,
            signature: &str,
        ) -> Result<(), DomainError>;

    }
}

mock!{
    pub SupportRepo{}

    #[async_trait]

    impl SupportRepository for SupportRepo {

        async fn persist_complaint(&self, complaint: &Complaint) -> Result<(), RepositoryError>;
        async fn persist_comment(
            &self,
            complaint_id: Uuid,
            comment: serde_json::Value,
        ) -> Result<(), RepositoryError>;
    
        async fn get_complaint_by_id(&self, id: Uuid) -> Result<Option<Complaint>, RepositoryError>;
        async fn get_complaint_by_status(
            &self,
            status: &SupportStatus,
        ) -> Result<Vec<Complaint>, RepositoryError>;
    
        async fn get_all_compalints(&self) -> Result<Vec<Complaint>, RepositoryError>;
    
        async fn update_complaint_status(
            &self,
            status: &SupportStatus,
            complaint: &Complaint,
        ) -> Result<(), RepositoryError>;
        async fn delete_complaint(&self, id: Uuid) -> Result<u64, RepositoryError>;
    }
}