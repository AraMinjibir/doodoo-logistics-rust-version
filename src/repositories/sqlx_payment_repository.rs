use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::domain::{models::payment::Payment,
    errors::{repository_error::RepositoryError, repository_error::map_sqlx_error},
    models::{payment_status::PaymentStatus}
};

use crate::repositories::payment_repository::PaymentRepository;
use crate::infrastructure::payment_row::PaymentRow;
pub struct SqlxPaymentRepository {
    pool: PgPool,
}

impl SqlxPaymentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PaymentRepository for SqlxPaymentRepository {
    async fn persist_payment(&self, payment: &Payment) -> Result<(), RepositoryError> {
        // 1. Transform Domain Model to Infrastructure Row
        let row = PaymentRow::from_domain(payment);

        // 2. Execute the Insert using SQLx
        sqlx::query!(
            r#"
            INSERT INTO payments (
                reference_number, customer_id, shipment_id, amount, 
                status, payment_method, paid_at, 
                gateway_transaction_id, failure_reason
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            row.reference_number,
            row.customer_id,
            row.shipment_id,
            row.amount,
            row.status.to_string(),
            row.payment_method.to_string(),
            row.paid_at,
            row.gateway_transaction_id,
            row.failure_reason
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn get_payment_by_ref(&self, reference: &str) -> Result<Option<Payment>, RepositoryError> {
        // 1. Fetch the optional row from the database
        let row: Option<PaymentRow> = sqlx::query_as!(
            PaymentRow,
            r#"
            SELECT 
                reference_number, customer_id, shipment_id, amount, 
                status, payment_method, paid_at, 
                gateway_transaction_id, failure_reason
            FROM payments
            WHERE reference_number = $1
            "#,
            reference
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        // 2. Map the Infrastructure Row back to the Domain Model
        Ok(row.map(|r| r.into_domain()))
    }

    async fn get_payment_by_status(&self, status: &str) -> Result<Vec<Payment>, RepositoryError> {

        let rows: Vec<PaymentRow> = sqlx::query_as!(
            PaymentRow,
            r#"
            SELECT * FROM payments WHERE status = $1
            "#,
            status
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(rows.into_iter().map(|r| r.into_domain()).collect())
    }

    async fn get_payment_by_shipment_id(&self, shipment_id: Uuid) -> Result<Option<Payment>, RepositoryError> {
        let row: Option<PaymentRow> = sqlx::query_as!(
            PaymentRow,
            r#"
            SELECT * FROM payments WHERE shipment_id = $1
            "#,
            shipment_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(row.map(|r| r.into_domain()))
    }

    async fn get_all_payments(&self) -> Result<Vec<Payment>, RepositoryError> {
        let rows: Vec<PaymentRow> = sqlx::query_as!(
            PaymentRow,
            r#"
            SELECT * FROM payments ORDER BY paid_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(rows.into_iter().map(|r| r.into_domain()).collect())
    }

    async fn update_payment(&self, payment: &Payment) -> Result<(), RepositoryError> {
        let row = PaymentRow::from_domain(payment);

        let result = sqlx::query!(
            r#"
            UPDATE payments 
            SET status = $1, 
                gateway_transaction_id = $2, 
                failure_reason = $3
            WHERE reference_number = $4
            "#,
            row.status.to_string(),
            row.gateway_transaction_id,
            row.failure_reason,
            row.reference_number
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        if result.rows_affected() == 0 {
            // Return a specific 'NotFound' error here
            return Err(RepositoryError::DatabaseError("No payment found to update".into()));
        }

        Ok(())
    }

    async fn delete_payment(&self, reference: &str) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM payments 
            WHERE reference_number = $1
            "#,
            reference
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn get_daily_revenue(
        &self,
        date: NaiveDate,
    ) -> Result<Option<Decimal>, RepositoryError> {
        let total = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(amount), 0) as "total!"
            FROM payments
            WHERE status = 'success'
              AND paid_at::date = $1
            "#,
            date
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
    
        Ok(Some(total))
    }

    async fn get_weekly_revenue(
        &self,
        date: NaiveDate,
    ) -> Result<Option<Decimal>, RepositoryError> {
        let start = date.and_hms_opt(0, 0, 0).unwrap();
    
        let total = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(amount), 0) as "total!"
            FROM payments
            WHERE status = 'success'
              AND paid_at >= date_trunc('week', $1::timestamp)
              AND paid_at < (date_trunc('week', $1::timestamp) + interval '1 week')
            "#,
            start
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
    
        Ok(Some(total))
    }
    
    async fn get_monthly_revenue(
        &self,
        year: u32,
        month: u32,
    ) -> Result<Option<Decimal>, RepositoryError> {
        let total = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(amount), 0) as "total!"
            FROM payments
            WHERE status = 'success'
              AND EXTRACT(YEAR FROM paid_at)::int = $1
              AND EXTRACT(MONTH FROM paid_at)::int = $2
            "#,
            year as i32,
            month as i32
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
    
        Ok(Some(total))
    }
}