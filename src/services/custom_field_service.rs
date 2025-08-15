use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::dto::custom_field_dto::{CreateCustomFieldRequest, CustomFieldResponse};
use crate::errors::AppError;
use crate::repositories::CustomFieldRepository;

pub struct CustomFieldService;

impl CustomFieldService {
    pub async fn create_custom_field(
        pool: &PgPool,
        request: CreateCustomFieldRequest,
        created_by: Uuid,
    ) -> Result<CustomFieldResponse, AppError> {
        // Validate the request
        if let Err(validation_errors) = request.validate() {
            tracing::warn!("Custom field validation failed: {:?}", validation_errors);
            return Err(AppError::ValidationError(
                "Invalid custom field data".to_string()
            ));
        }

        // Validate field type
        let valid_field_types = vec![
            "text", "textarea", "number", "email", "phone", "date", 
            "boolean", "select", "multi_select"
        ];
        
        if !valid_field_types.contains(&request.field_type.as_str()) {
            return Err(AppError::ValidationError(
                format!("Invalid field type. Must be one of: {}", valid_field_types.join(", "))
            ));
        }

        // Check if field already exists for this module
        if CustomFieldRepository::field_exists(pool, &request.module, &request.field_name).await? {
            return Err(AppError::ValidationError(
                "A custom field with this name already exists for this module".to_string()
            ));
        }

        // Validate options for select fields
        if (request.field_type == "select" || request.field_type == "multi_select") && request.options.is_none() {
            return Err(AppError::ValidationError(
                "Options are required for select and multi_select field types".to_string()
            ));
        }

        // Create the custom field
        let custom_field = CustomFieldRepository::create(
            pool,
            request.module,
            request.label,
            request.field_name,
            request.field_type,
            request.is_required.unwrap_or(false),
            request.is_active.unwrap_or(true),
            request.options,
            request.validation_rules,
            request.default_value,
            request.help_text,
            request.display_order.unwrap_or(0),
            Some(created_by),
        ).await?;

        // Convert to response DTO
        Ok(Self::to_response(custom_field))
    }

    pub async fn get_custom_fields_by_module(
        pool: &PgPool,
        module: &str,
    ) -> Result<Vec<CustomFieldResponse>, AppError> {
        let custom_fields = CustomFieldRepository::get_by_module(pool, module).await?;
        
        Ok(custom_fields.into_iter().map(Self::to_response).collect())
    }

    fn to_response(custom_field: crate::models::CustomField) -> CustomFieldResponse {
        CustomFieldResponse {
            id: custom_field.id,
            module: custom_field.module,
            label: custom_field.label,
            field_name: custom_field.field_name,
            field_type: custom_field.field_type,
            is_required: custom_field.is_required,
            is_active: custom_field.is_active,
            options: custom_field.options,
            validation_rules: custom_field.validation_rules,
            default_value: custom_field.default_value,
            help_text: custom_field.help_text,
            display_order: custom_field.display_order,
            created_by: custom_field.created_by,
            created_at: custom_field.created_at.to_rfc3339(),
            updated_at: custom_field.updated_at.to_rfc3339(),
        }
    }
}