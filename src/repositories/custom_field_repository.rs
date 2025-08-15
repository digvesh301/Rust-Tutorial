use sqlx::PgPool;
use uuid::Uuid;
use serde_json::Value as JsonValue;

use crate::errors::AppError;
use crate::models::CustomField;

pub struct CustomFieldRepository;

impl CustomFieldRepository {
    pub async fn create(
        pool: &PgPool,
        module: String,
        label: String,
        field_name: String,
        field_type: String,
        is_required: bool,
        is_active: bool,
        options: Option<JsonValue>,
        validation_rules: Option<JsonValue>,
        default_value: Option<String>,
        help_text: Option<String>,
        display_order: i32,
        created_by: Option<Uuid>,
    ) -> Result<CustomField, AppError> {
        let custom_field = sqlx::query_as!(
            CustomField,
            r#"
            INSERT INTO custom_fields (
                module, label, field_name, field_type, is_required, is_active,
                options, validation_rules, default_value, help_text, display_order, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
            module,
            label,
            field_name,
            field_type,
            is_required,
            is_active,
            options,
            validation_rules,
            default_value,
            help_text,
            display_order,
            created_by
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create custom field: {:?}", e);
            if e.to_string().contains("duplicate key") {
                AppError::ValidationError("A custom field with this name already exists for this module".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(custom_field)
    }

    pub async fn field_exists(
        pool: &PgPool,
        module: &str,
        field_name: &str,
    ) -> Result<bool, AppError> {
        let exists = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM custom_fields WHERE module = $1 AND field_name = $2)",
            module,
            field_name
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(exists.exists.unwrap_or(false))
    }

    pub async fn get_by_module(
        pool: &PgPool,
        module: &str,
    ) -> Result<Vec<CustomField>, AppError> {
        let custom_fields = sqlx::query_as!(
            CustomField,
            "SELECT * FROM custom_fields WHERE module = $1 AND is_active = true ORDER BY display_order ASC",
            module
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(custom_fields)
    }
}