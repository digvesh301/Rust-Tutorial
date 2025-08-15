use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCustomFieldRequest {
    #[validate(length(min = 1, max = 50, message = "Module must be between 1 and 50 characters"))]
    pub module: String,
    
    #[validate(length(min = 1, max = 255, message = "Label must be between 1 and 255 characters"))]
    pub label: String,
    
    #[validate(length(min = 1, max = 100, message = "Field name must be between 1 and 100 characters"))]
    #[validate(regex(path = "crate::utils::validation::FIELD_NAME_REGEX", message = "Field name must be snake_case"))]
    pub field_name: String,
    
    #[validate(length(min = 1, max = 50, message = "Field type is required"))]
    pub field_type: String, // 'text', 'number', 'email', 'phone', 'date', 'boolean', 'select', 'multi_select', 'textarea'
    
    pub is_required: Option<bool>,
    pub is_active: Option<bool>,
    pub options: Option<JsonValue>, // For select/multi_select fields
    pub validation_rules: Option<JsonValue>,
    pub default_value: Option<String>,
    pub help_text: Option<String>,
    pub display_order: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CustomFieldResponse {
    pub id: Uuid,
    pub module: String,
    pub label: String,
    pub field_name: String,
    pub field_type: String,
    pub is_required: bool,
    pub is_active: bool,
    pub options: Option<JsonValue>,
    pub validation_rules: Option<JsonValue>,
    pub default_value: Option<String>,
    pub help_text: Option<String>,
    pub display_order: i64,
    pub created_by: Option<Uuid>,
    pub created_at: String,
    pub updated_at: String,
}
