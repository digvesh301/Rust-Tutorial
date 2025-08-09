use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};

use crate::dto::contact_dto::CreateContactRequest;
use crate::errors::AppError;
use crate::middleware::check_user_permission;
use crate::services::contact_service::ContactService;
use crate::AppState;
use uuid::Uuid;

/// Create a new contact with permission checking using middleware
/// POST /api/contacts
pub async fn create_contact(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(request): Json<CreateContactRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    // Use permission middleware to check permission and get user (handles token extraction internally)
    let user = check_user_permission(&state, &headers, "contacts:create").await?;

    tracing::info!(
        "Creating contact: {} {} by user: {} (permission verified via middleware)",
        request.first_name,
        request.last_name,
        user.id
    );

    let contact = ContactService::create_contact(&state.db, request, user.id).await?;

    let response = json!({
        "success": true,
        "message": "Contact created successfully",
        "data": contact
    });

    Ok((StatusCode::CREATED, Json(response)))
}

/// View a single contact by ID with permission checking
/// GET /api/contacts/:id
pub async fn get_contact(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(contact_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    // Permission: read contacts
    let user = check_user_permission(&state, &headers, "contacts:read").await?;

    let contact = ContactService::get_contact_by_id(&state.db, contact_id, user.id).await?;

    let response = json!({
        "success": true,
        "data": contact
    });

    Ok(Json(response))
}






