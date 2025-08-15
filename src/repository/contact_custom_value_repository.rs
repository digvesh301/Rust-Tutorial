use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{ContactCustomValue, CustomField};

pub struct ContactCustomValueRepository;

impl ContactCustomValueRepository {
    /// Create a new contact custom value
    pub async fn create(pool: &PgPool, custom_value: &ContactCustomValue) -> Result<ContactCustomValue, AppError> {
        let query = r#"
            INSERT INTO contact_custom_values (
                id, contact_id, custom_field_id, value, value_json, 
                value_number, value_date, value_boolean, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
        "#;

        let result = sqlx::query_as::<_, ContactCustomValue>(query)
            .bind(&custom_value.id)
            .bind(&custom_value.contact_id)
            .bind(&custom_value.custom_field_id)
            .bind(&custom_value.value)
            .bind(&custom_value.value_json)
            .bind(&custom_value.value_number)
            .bind(&custom_value.value_date)
            .bind(&custom_value.value_boolean)
            .bind(&custom_value.created_at)
            .bind(&custom_value.updated_at)
            .fetch_one(pool)
            .await;

        match result {
            Ok(custom_value) => {
                tracing::info!("Contact custom value created successfully with ID: {}", custom_value.id);
                Ok(custom_value)
            }
            Err(e) => {
                tracing::error!("Error creating contact custom value: {}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Get all custom values for a contact
    pub async fn find_by_contact_id(pool: &PgPool, contact_id: Uuid) -> Result<Vec<ContactCustomValue>, AppError> {
        let query = r#"
            SELECT * FROM contact_custom_values 
            WHERE contact_id = $1
            ORDER BY created_at ASC
        "#;

        let result = sqlx::query_as::<_, ContactCustomValue>(query)
            .bind(contact_id)
            .fetch_all(pool)
            .await;

        match result {
            Ok(custom_values) => Ok(custom_values),
            Err(e) => {
                tracing::error!("Error finding custom values for contact {}: {}", contact_id, e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Get custom fields for contact module
    pub async fn get_contact_custom_fields(pool: &PgPool) -> Result<Vec<CustomField>, AppError> {
        let query = r#"
            SELECT * FROM custom_fields 
            WHERE module = 'contact' AND is_active = true
            ORDER BY display_order ASC, label ASC
        "#;

        let result = sqlx::query_as::<_, CustomField>(query)
            .fetch_all(pool)
            .await;

        match result {
            Ok(fields) => Ok(fields),
            Err(e) => {
                tracing::error!("Error getting contact custom fields: {}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Find custom field by field name
    pub async fn find_custom_field_by_name(pool: &PgPool, field_name: &str) -> Result<Option<CustomField>, AppError> {
        let query = r#"
            SELECT * FROM custom_fields 
            WHERE module = 'contact' AND field_name = $1 AND is_active = true
        "#;

        let result = sqlx::query_as::<_, CustomField>(query)
            .bind(field_name)
            .fetch_optional(pool)
            .await;

        match result {
            Ok(field) => Ok(field),
            Err(e) => {
                tracing::error!("Error finding custom field by name {}: {}", field_name, e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Update or create custom value (upsert)
    pub async fn upsert(pool: &PgPool, custom_value: &ContactCustomValue) -> Result<ContactCustomValue, AppError> {
        let query = r#"
            INSERT INTO contact_custom_values (
                id, contact_id, custom_field_id, value, value_json, 
                value_number, value_date, value_boolean, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (contact_id, custom_field_id) 
            DO UPDATE SET
                value = EXCLUDED.value,
                value_json = EXCLUDED.value_json,
                value_number = EXCLUDED.value_number,
                value_date = EXCLUDED.value_date,
                value_boolean = EXCLUDED.value_boolean,
                updated_at = NOW()
            RETURNING *
        "#;

        let result = sqlx::query_as::<_, ContactCustomValue>(query)
            .bind(&custom_value.id)
            .bind(&custom_value.contact_id)
            .bind(&custom_value.custom_field_id)
            .bind(&custom_value.value)
            .bind(&custom_value.value_json)
            .bind(&custom_value.value_number)
            .bind(&custom_value.value_date)
            .bind(&custom_value.value_boolean)
            .bind(&custom_value.created_at)
            .bind(&custom_value.updated_at)
            .fetch_one(pool)
            .await;

        match result {
            Ok(custom_value) => {
                tracing::info!("Contact custom value upserted successfully with ID: {}", custom_value.id);
                Ok(custom_value)
            }
            Err(e) => {
                tracing::error!("Error upserting contact custom value: {}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Delete a custom field value for a specific contact and field
    pub async fn delete_by_contact_and_field(
        pool: &PgPool,
        contact_id: Uuid,
        custom_field_id: Uuid,
    ) -> Result<(), AppError> {
        let query = r#"
            DELETE FROM contact_custom_values
            WHERE contact_id = $1 AND custom_field_id = $2
        "#;

        let result = sqlx::query(query)
            .bind(contact_id)
            .bind(custom_field_id)
            .execute(pool)
            .await;

        match result {
            Ok(query_result) => {
                tracing::info!(
                    "Deleted {} custom field value(s) for contact {} and field {}",
                    query_result.rows_affected(),
                    contact_id,
                    custom_field_id
                );
                Ok(())
            }
            Err(e) => {
                tracing::error!(
                    "Error deleting custom field value for contact {} and field {}: {}",
                    contact_id,
                    custom_field_id,
                    e
                );
                Err(AppError::DatabaseError(e))
            }
        }
    }
}
