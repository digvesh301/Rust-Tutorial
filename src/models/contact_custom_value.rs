use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ContactCustomValue {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub custom_field_id: Uuid,
    pub value: Option<String>,
    pub value_json: Option<JsonValue>,
    pub value_number: Option<BigDecimal>,
    pub value_date: Option<chrono::NaiveDate>,
    pub value_boolean: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ContactCustomValue {
    pub fn new(contact_id: Uuid, custom_field_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            contact_id,
            custom_field_id,
            value: None,
            value_json: None,
            value_number: None,
            value_date: None,
            value_boolean: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set value based on field type
    pub fn set_value(&mut self, field_type: &str, value: &str) -> Result<(), String> {
        match field_type {
            "text" | "email" | "phone" | "textarea" | "select" => {
                self.value = Some(value.to_string());
            }
            "number" => {
                match value.parse::<BigDecimal>() {
                    Ok(num) => {
                        self.value_number = Some(num);
                        self.value = Some(value.to_string());
                    }
                    Err(_) => return Err("Invalid number format".to_string()),
                }
            }
            "boolean" => {
                match value.to_lowercase().as_str() {
                    "true" | "1" | "yes" => {
                        self.value_boolean = Some(true);
                        self.value = Some("true".to_string());
                    }
                    "false" | "0" | "no" => {
                        self.value_boolean = Some(false);
                        self.value = Some("false".to_string());
                    }
                    _ => return Err("Invalid boolean format".to_string()),
                }
            }
            "date" => {
                match chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                    Ok(date) => {
                        self.value_date = Some(date);
                        self.value = Some(value.to_string());
                    }
                    Err(_) => return Err("Invalid date format (expected YYYY-MM-DD)".to_string()),
                }
            }
            "multi_select" => {
                // Parse as JSON array
                match serde_json::from_str::<Vec<String>>(value) {
                    Ok(array) => {
                        self.value_json = Some(serde_json::to_value(array).unwrap());
                        self.value = Some(value.to_string());
                    }
                    Err(_) => return Err("Invalid multi_select format (expected JSON array)".to_string()),
                }
            }
            _ => {
                // Default to text
                self.value = Some(value.to_string());
            }
        }
        Ok(())
    }
}
