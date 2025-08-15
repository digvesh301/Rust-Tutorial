use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::dto::custom_field_dto::CreateCustomFieldRequest;
use crate::errors::AppError;
use crate::middleware::check_user_permission;
use crate::services::CustomFieldService;
use crate::AppState;

/// Create a new custom field
/// POST /api/custom-fields
pub async fn create_custom_field(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(request): Json<CreateCustomFieldRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    // Check permission
    let user = check_user_permission(&state, &headers, "custom_fields:create").await?;

    tracing::info!(
        "Creating custom field: {} for module: {} by user: {}",
        request.field_name,
        request.module,
        user.id
    );

    let custom_field = CustomFieldService::create_custom_field(&state.db, request, user.id).await?;

    let response = json!({
        "success": true,
        "message": "Custom field created successfully",
        "data": custom_field
    });

    Ok((StatusCode::CREATED, Json(response)))
}

/// Get custom fields by module
/// GET /api/custom-fields/:module
pub async fn get_custom_fields_by_module(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(module): Path<String>,
) -> Result<Json<Value>, AppError> {
    // Check permission
    let _user = check_user_permission(&state, &headers, "custom_fields:read").await?;

    let custom_fields = CustomFieldService::get_custom_fields_by_module(&state.db, &module).await?;

    let response = json!({
        "success": true,
        "data": custom_fields
    });

    Ok(Json(response))
}