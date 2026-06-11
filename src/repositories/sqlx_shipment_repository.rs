use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::repository_error::map_sqlx_error;
use crate::domain::errors::repository_error::RepositoryError;
use crate::domain::models::shipment::Shipment;
use crate::infrastructure::mappers::shipment_mapper::ShipmentMapper;
use crate::infrastructure::shipment_row::ShipmentRow;
use crate::repositories::shipment_repository::ShipmentRepository;

pub struct SqlxShipmentRepository {
    pool: PgPool,
}

impl SqlxShipmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ShipmentRepository for SqlxShipmentRepository {
    async fn create(&self, shipment: &Shipment) -> Result<(), RepositoryError> {
        let row = ShipmentMapper::to_row(shipment.clone());

        sqlx::query!(
            r#"INSERT INTO shipments (
                id, tracking_number, sender_name,
                recipient_name, recipient_street, recipient_city,
                recipient_state, recipient_country, recipient_postal_code,
                recipient_contact,
                weight, length, width, height,
                contents, status,
                estimated_delivery_date,
                created_at, updated_at,
                cost,
                proof_of_delivery,
                service_provider_id
            ) VALUES (
                $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,
                $11,$12,$13,$14,$15,$16,
                $17,$18,$19,$20,$21,$22
            )"#,
            row.id,
            row.tracking_number,
            row.sender_name,
            row.recipient_name,
            row.recipient_street,
            row.recipient_city,
            row.recipient_state,
            row.recipient_country,
            row.recipient_postal_code,
            row.recipient_contact,
            row.weight,
            row.length,
            row.width,
            row.height,
            row.contents,
            row.status,
            row.estimated_delivery_date,
            row.created_at,
            row.updated_at,
            row.cost,
            row.proof_of_delivery,
            row.service_provider_id
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
    async fn update(&self, shipment: &Shipment) -> Result<(), RepositoryError> {
        let row = ShipmentMapper::to_row(shipment.clone());

        sqlx::query!(
            r#"UPDATE shipments SET
                tracking_number = $1,
                sender_name = $2,
                recipient_name = $3,
                recipient_street = $4,
                recipient_city = $5,
                recipient_state = $6,
                recipient_country = $7,
                recipient_postal_code = $8,
                recipient_contact = $9,
                weight = $10,
                length = $11,
                width = $12,
                height = $13,
                contents = $14,
                status = $15,
                updated_at = $16,
                cost = $17,
                proof_of_delivery = $18,
                service_provider_id = $19
            WHERE id = $20"#,
            row.tracking_number,
            row.sender_name,
            row.recipient_name,
            row.recipient_street,
            row.recipient_city,
            row.recipient_state,
            row.recipient_country,
            row.recipient_postal_code,
            row.recipient_contact,
            row.weight,
            row.length,
            row.width,
            row.height,
            row.contents,
            row.status,
            row.updated_at,
            row.cost,
            row.proof_of_delivery,
            row.service_provider_id,
            row.id
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
    async fn delete(&self, id: Uuid) -> Result<u64, RepositoryError> {
        let result = sqlx::query!("DELETE FROM shipments WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(result.rows_affected())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Shipment>, RepositoryError> {
        let row = sqlx::query_as!(ShipmentRow, "SELECT * FROM shipments WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(row.map(ShipmentMapper::from_row))
    }

    async fn get_by_status(&self, status: &str) -> Result<Vec<Shipment>, RepositoryError> {
        let rows = sqlx::query_as!(
            ShipmentRow,
            "SELECT * FROM shipments WHERE status = $1",
            status
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        let shipments = rows.into_iter().map(ShipmentMapper::from_row).collect();

        Ok(shipments)
    }
    async fn find_by_tracking_number(
        &self,
        tracking: &str,
    ) -> Result<Option<Shipment>, RepositoryError> {
        let row = sqlx::query_as!(
            ShipmentRow,
            "SELECT * FROM shipments WHERE tracking_number = $1",
            tracking
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(row.map(ShipmentMapper::from_row))
    }

    async fn list_all(&self, offset: i64, limit: i64) -> Result<Vec<Shipment>, RepositoryError> {
        let rows = sqlx::query_as!(
            ShipmentRow,
            "SELECT * FROM shipments ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        let shipments = rows.into_iter().map(ShipmentMapper::from_row).collect();

        Ok(shipments)
    }

    async fn upload_proof_of_delivery(
        &self,
        shipment_id: Uuid,
        proof: serde_json::Value,
    ) -> Result<Option<Shipment>, RepositoryError> {
        sqlx::query!(
            r#"
            UPDATE shipments
            SET proof_of_delivery = $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
            proof,
            shipment_id
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        self.get_by_id(shipment_id).await
    }

    async fn assign_service_provider(
        &self,
        shipment_id: Uuid,
        provider_id: Uuid,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            "UPDATE shipments SET service_provider_id = $1 WHERE id = $2",
            provider_id,
            shipment_id
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn find_by_service_provider(
        &self,
        provider_id: Uuid,
    ) -> Result<Vec<Shipment>, RepositoryError> {
        let rows = sqlx::query_as!(
            ShipmentRow,
            "SELECT * FROM shipments WHERE service_provider_id = $1",
            provider_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(rows.into_iter().map(ShipmentMapper::from_row).collect())
    }
}
