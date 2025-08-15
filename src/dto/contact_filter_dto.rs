use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogicOperator {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    // Text operators
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    IsEmpty,
    IsNotEmpty,
    
    // Number operators
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Between,
    
    // Date operators
    After,
    Before,
    
    // Select operators
    In,
    NotIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Text,
    Number,
    Date,
    Select,
    Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FilterNode {
    Condition {
        field: String,
        operator: FilterOperator,
        value: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_type: Option<FieldType>,
    },
    Group {
        logic: LogicOperator,
        conditions: Vec<FilterNode>,
    },
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ContactFilterRequest {
    pub logic: LogicOperator,
    pub conditions: Vec<FilterNode>,
    
    // Pagination
    #[serde(default = "default_page")]
    pub page: u32,
    
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 1000, message = "Limit must be between 1 and 1000"))]
    pub limit: u32,
    
    // Sorting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    
    #[serde(default = "default_sort_order")]
    pub sort_order: SortOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactFilterResponse {
    pub success: bool,
    pub data: Vec<ContactSummary>,
    pub pagination: PaginationInfo,
    pub total_count: u64,
    pub filter_summary: FilterSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactSummary {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub lead_status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterSummary {
    pub total_conditions: u32,
    pub fields_used: Vec<String>,
    pub custom_fields_used: Vec<String>,
    pub execution_time_ms: u64,
}

// Default functions
fn default_page() -> u32 { 1 }
fn default_limit() -> u32 { 50 }
fn default_sort_order() -> SortOrder { SortOrder::Desc }

// Standard contact fields that are stored in the main table
pub const STANDARD_FIELDS: &[&str] = &[
    "id", "first_name", "last_name", "email", "phone", "company", 
    "job_title", "address", "city", "state", "postal_code", "country",
    "notes", "lead_source", "lead_status", "owner_id", "is_active",
    "created_at", "updated_at"
];

impl ContactFilterRequest {
    pub fn new(logic: LogicOperator, conditions: Vec<FilterNode>) -> Self {
        Self {
            logic,
            conditions,
            page: default_page(),
            limit: default_limit(),
            sort_by: None,
            sort_order: default_sort_order(),
        }
    }
    
    pub fn with_pagination(mut self, page: u32, limit: u32) -> Self {
        self.page = page;
        self.limit = limit;
        self
    }
    
    pub fn with_sorting(mut self, sort_by: String, sort_order: SortOrder) -> Self {
        self.sort_by = Some(sort_by);
        self.sort_order = sort_order;
        self
    }
}

impl FilterNode {
    pub fn condition(field: String, operator: FilterOperator, value: serde_json::Value) -> Self {
        Self::Condition {
            field,
            operator,
            value,
            field_type: None,
        }
    }
    
    pub fn condition_with_type(
        field: String, 
        operator: FilterOperator, 
        value: serde_json::Value,
        field_type: FieldType
    ) -> Self {
        Self::Condition {
            field,
            operator,
            value,
            field_type: Some(field_type),
        }
    }
    
    pub fn group(logic: LogicOperator, conditions: Vec<FilterNode>) -> Self {
        Self::Group { logic, conditions }
    }
    
    pub fn and_group(conditions: Vec<FilterNode>) -> Self {
        Self::Group { 
            logic: LogicOperator::And, 
            conditions 
        }
    }
    
    pub fn or_group(conditions: Vec<FilterNode>) -> Self {
        Self::Group { 
            logic: LogicOperator::Or, 
            conditions 
        }
    }
}

impl PaginationInfo {
    pub fn new(page: u32, limit: u32, total_count: u64) -> Self {
        let total_pages = ((total_count as f64) / (limit as f64)).ceil() as u32;
        let has_next = page < total_pages;
        let has_prev = page > 1;
        
        Self {
            page,
            limit,
            total_pages,
            has_next,
            has_prev,
        }
    }
}

// Helper functions for creating common filter conditions
pub mod filters {
    use super::*;
    
    pub fn name_equals(name: &str) -> FilterNode {
        FilterNode::condition(
            "first_name".to_string(),
            FilterOperator::Equals,
            serde_json::Value::String(name.to_string())
        )
    }
    
    pub fn email_contains(email: &str) -> FilterNode {
        FilterNode::condition(
            "email".to_string(),
            FilterOperator::Contains,
            serde_json::Value::String(email.to_string())
        )
    }
    
    pub fn company_in(companies: Vec<String>) -> FilterNode {
        FilterNode::condition(
            "company".to_string(),
            FilterOperator::In,
            serde_json::Value::Array(
                companies.into_iter()
                    .map(serde_json::Value::String)
                    .collect()
            )
        )
    }
    
    pub fn custom_field_equals(field: &str, value: &str) -> FilterNode {
        FilterNode::condition_with_type(
            field.to_string(),
            FilterOperator::Equals,
            serde_json::Value::String(value.to_string()),
            FieldType::Text
        )
    }
    
    pub fn custom_field_number_greater(field: &str, value: f64) -> FilterNode {
        FilterNode::condition_with_type(
            field.to_string(),
            FilterOperator::GreaterThan,
            serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap()),
            FieldType::Number
        )
    }
    
    pub fn lead_status_in(statuses: Vec<String>) -> FilterNode {
        FilterNode::condition(
            "lead_status".to_string(),
            FilterOperator::In,
            serde_json::Value::Array(
                statuses.into_iter()
                    .map(serde_json::Value::String)
                    .collect()
            )
        )
    }
}
