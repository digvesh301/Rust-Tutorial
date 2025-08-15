use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CustomField {
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CustomField {
    pub fn new(
        module: String,
        label: String,
        field_name: String,
        field_type: String,
        created_by: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            module,
            label,
            field_name,
            field_type,
            is_required: false,
            is_active: true,
            options: None,
            validation_rules: None,
            default_value: None,
            help_text: None,
            display_order: 0,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }
}