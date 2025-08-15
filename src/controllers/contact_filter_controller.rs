use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
};
use validator::Validate;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::dto::contact_filter_dto::*;
use crate::errors::AppError;
use crate::middleware::permission_middleware::check_user_permission;
use crate::services::contact_filter_service::ContactFilterService;
use crate::AppState;

/// Filter contacts with complex nested conditions
/// POST /api/contacts/filter
pub async fn filter_contacts(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(filter_request): Json<ContactFilterRequest>,
) -> Result<Json<Value>, AppError> {
    // Permission: read contacts
    let user = check_user_permission(&state, &headers, "contacts:read").await?;

    tracing::info!(
        "Filtering contacts with {} conditions by user: {} (permission verified via middleware)",
        filter_request.conditions.len(),
        user.id
    );

    let response = ContactFilterService::filter_contacts(&state.db, filter_request).await?;

    Ok(Json(json!(response)))
}

/// Get available filter fields and their types
/// GET /api/contacts/filter/fields
pub async fn get_filter_fields(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    // Permission: read contacts
    let _user = check_user_permission(&state, &headers, "contacts:read").await?;

    // Get standard fields
    let standard_fields = get_standard_field_definitions();
    
    // Get custom fields from database
    let custom_fields = get_custom_field_definitions(&state.db).await?;

    let response = json!({
        "success": true,
        "data": {
            "standard_fields": standard_fields,
            "custom_fields": custom_fields
        }
    });

    Ok(Json(response))
}

/// Get filter presets/templates
/// GET /api/contacts/filter/presets
pub async fn get_filter_presets(
    State(_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    // Permission: read contacts
    let _user = check_user_permission(&_state, &headers, "contacts:read").await?;

    let presets = get_common_filter_presets();

    let response = json!({
        "success": true,
        "data": presets
    });

    Ok(Json(response))
}

/// Validate a filter structure without executing it
/// POST /api/contacts/filter/validate
pub async fn validate_filter(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Json(filter_request): Json<ContactFilterRequest>,
) -> Result<Json<Value>, AppError> {
    // Permission: read contacts
    let _user = check_user_permission(&_state, &headers, "contacts:read").await?;

    // Validate the filter structure
    match filter_request.validate() {
        Ok(_) => {
            // Additional custom validation
            let validation_result = validate_filter_logic(&filter_request)?;
            
            Ok(Json(json!({
                "success": true,
                "message": "Filter is valid",
                "data": validation_result
            })))
        }
        Err(e) => {
            Ok(Json(json!({
                "success": false,
                "message": "Filter validation failed",
                "errors": e.to_string()
            })))
        }
    }
}

// Helper functions

fn get_standard_field_definitions() -> HashMap<String, FieldDefinition> {
    let mut fields = HashMap::new();
    
    // Text fields
    for field in &["first_name", "last_name", "email", "company", "job_title", "address", "city", "state", "postal_code", "country", "notes", "lead_source"] {
        fields.insert(field.to_string(), FieldDefinition {
            name: field.to_string(),
            field_type: FieldType::Text,
            label: field.replace('_', " ").to_title_case(),
            operators: vec![
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::Contains,
                FilterOperator::StartsWith,
                FilterOperator::EndsWith,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
                FilterOperator::In,
                FilterOperator::NotIn,
            ],
            required: field == &"first_name" || field == &"last_name" || field == &"email",
        });
    }
    
    // Phone field (special text field)
    fields.insert("phone".to_string(), FieldDefinition {
        name: "phone".to_string(),
        field_type: FieldType::Text,
        label: "Phone".to_string(),
        operators: vec![
            FilterOperator::Equals,
            FilterOperator::NotEquals,
            FilterOperator::Contains,
            FilterOperator::StartsWith,
            FilterOperator::IsEmpty,
            FilterOperator::IsNotEmpty,
        ],
        required: false,
    });
    
    // Select field
    fields.insert("lead_status".to_string(), FieldDefinition {
        name: "lead_status".to_string(),
        field_type: FieldType::Select,
        label: "Lead Status".to_string(),
        operators: vec![
            FilterOperator::Equals,
            FilterOperator::NotEquals,
            FilterOperator::In,
            FilterOperator::NotIn,
        ],
        required: false,
    });
    
    // Date fields
    for field in &["created_at", "updated_at"] {
        fields.insert(field.to_string(), FieldDefinition {
            name: field.to_string(),
            field_type: FieldType::Date,
            label: field.replace('_', " ").to_title_case(),
            operators: vec![
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::After,
                FilterOperator::Before,
                FilterOperator::Between,
            ],
            required: false,
        });
    }
    
    // Boolean field
    fields.insert("is_active".to_string(), FieldDefinition {
        name: "is_active".to_string(),
        field_type: FieldType::Boolean,
        label: "Is Active".to_string(),
        operators: vec![
            FilterOperator::Equals,
            FilterOperator::NotEquals,
        ],
        required: false,
    });
    
    fields
}

async fn get_custom_field_definitions(pool: &sqlx::PgPool) -> Result<HashMap<String, FieldDefinition>, AppError> {
    let query = r#"
        SELECT field_name, field_type, is_required
        FROM custom_fields
        WHERE is_active = true
        ORDER BY field_name
    "#;
    
    let rows = sqlx::query_as::<_, CustomFieldRow>(query)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching custom fields: {}", e);
            AppError::DatabaseError(e)
        })?;
    
    let mut fields = HashMap::new();
    
    for row in rows {
        let field_type = match row.field_type.as_str() {
            "text" => FieldType::Text,
            "number" => FieldType::Number,
            "date" => FieldType::Date,
            "select" => FieldType::Select,
            "boolean" => FieldType::Boolean,
            _ => FieldType::Text, // Default to text
        };
        
        let operators = match field_type {
            FieldType::Text => vec![
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::Contains,
                FilterOperator::StartsWith,
                FilterOperator::EndsWith,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            FieldType::Number => vec![
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::GreaterThan,
                FilterOperator::LessThan,
                FilterOperator::GreaterEqual,
                FilterOperator::LessEqual,
                FilterOperator::Between,
            ],
            FieldType::Date => vec![
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::After,
                FilterOperator::Before,
                FilterOperator::Between,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            FieldType::Select => vec![
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::In,
                FilterOperator::NotIn,
            ],
            FieldType::Boolean => vec![
                FilterOperator::Equals,
                FilterOperator::NotEquals,
            ],
        };
        
        fields.insert(row.field_name.clone(), FieldDefinition {
            name: row.field_name.clone(),
            field_type,
            label: row.field_name.replace('_', " ").to_title_case(),
            operators,
            required: row.is_required,
        });
    }
    
    Ok(fields)
}

fn get_common_filter_presets() -> Vec<FilterPreset> {
    vec![
        FilterPreset {
            name: "Active Leads".to_string(),
            description: "Contacts with active lead status".to_string(),
            filter: ContactFilterRequest::new(
                LogicOperator::And,
                vec![
                    FilterNode::condition(
                        "lead_status".to_string(),
                        FilterOperator::In,
                        json!(["new", "contacted", "qualified"])
                    )
                ]
            ),
        },
        FilterPreset {
            name: "Recent Contacts".to_string(),
            description: "Contacts created in the last 30 days".to_string(),
            filter: ContactFilterRequest::new(
                LogicOperator::And,
                vec![
                    FilterNode::condition(
                        "created_at".to_string(),
                        FilterOperator::After,
                        json!("2024-01-01T00:00:00Z") // This would be dynamic in real implementation
                    )
                ]
            ),
        },
        FilterPreset {
            name: "Tech Companies".to_string(),
            description: "Contacts from technology companies".to_string(),
            filter: ContactFilterRequest::new(
                LogicOperator::Or,
                vec![
                    FilterNode::condition(
                        "company".to_string(),
                        FilterOperator::Contains,
                        json!("Tech")
                    ),
                    FilterNode::condition(
                        "company".to_string(),
                        FilterOperator::Contains,
                        json!("Software")
                    ),
                    FilterNode::condition(
                        "company".to_string(),
                        FilterOperator::Contains,
                        json!("IT")
                    ),
                ]
            ),
        },
    ]
}

fn validate_filter_logic(filter: &ContactFilterRequest) -> Result<FilterValidationResult, AppError> {
    let mut result = FilterValidationResult {
        is_valid: true,
        warnings: Vec::new(),
        suggestions: Vec::new(),
        estimated_performance: "good".to_string(),
    };
    
    // Check for potential performance issues
    let condition_count = count_total_conditions(&filter.conditions);
    if condition_count > 20 {
        result.warnings.push("Large number of conditions may impact performance".to_string());
        result.estimated_performance = "slow".to_string();
    }
    
    // Check for empty groups
    if has_empty_groups(&filter.conditions) {
        result.warnings.push("Filter contains empty groups".to_string());
    }
    
    // Suggest optimizations
    if has_redundant_conditions(&filter.conditions) {
        result.suggestions.push("Consider combining similar conditions".to_string());
    }
    
    Ok(result)
}

// Helper functions for validation
fn count_total_conditions(conditions: &[FilterNode]) -> usize {
    conditions.iter().map(|node| match node {
        FilterNode::Condition { .. } => 1,
        FilterNode::Group { conditions, .. } => count_total_conditions(conditions),
    }).sum()
}

fn has_empty_groups(conditions: &[FilterNode]) -> bool {
    conditions.iter().any(|node| match node {
        FilterNode::Condition { .. } => false,
        FilterNode::Group { conditions, .. } => {
            conditions.is_empty() || has_empty_groups(conditions)
        }
    })
}

fn has_redundant_conditions(_conditions: &[FilterNode]) -> bool {
    // Simplified check - in real implementation, this would be more sophisticated
    false
}

// Helper structs
#[derive(serde::Serialize, serde::Deserialize)]
struct FieldDefinition {
    name: String,
    field_type: FieldType,
    label: String,
    operators: Vec<FilterOperator>,
    required: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FilterPreset {
    name: String,
    description: String,
    filter: ContactFilterRequest,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FilterValidationResult {
    is_valid: bool,
    warnings: Vec<String>,
    suggestions: Vec<String>,
    estimated_performance: String,
}

#[derive(sqlx::FromRow)]
struct CustomFieldRow {
    field_name: String,
    field_type: String,
    is_required: bool,
}

// String extension trait for title case
trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for String {
    fn to_title_case(&self) -> String {
        self.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
