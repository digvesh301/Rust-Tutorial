use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};

use crate::dto::contact_dto::{CreateContactRequest, UpdateContactRequest, PatchContactRequest};
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

/// Update a contact by ID with permission checking
/// PUT /api/contacts/:id
pub async fn update_contact(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(contact_id): Path<Uuid>,
    Json(request): Json<UpdateContactRequest>,
) -> Result<Json<Value>, AppError> {
    // Permission: update contacts
    let user = check_user_permission(&state, &headers, "contacts:update").await?;

    tracing::info!(
        "Updating contact: {} by user: {} (permission verified via middleware)",
        contact_id,
        user.id
    );

    let contact = ContactService::update_contact(&state.db, contact_id, request, user.id).await?;

    let response = json!({
        "success": true,
        "message": "Contact updated successfully",
        "data": contact
    });

    Ok(Json(response))
}

/// Patch a contact by ID with permission checking (partial update with merge semantics)
/// PATCH /api/contacts/:id
pub async fn patch_contact(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(contact_id): Path<Uuid>,
    Json(request): Json<PatchContactRequest>,
) -> Result<Json<Value>, AppError> {
    // Permission: update contacts (same permission as PUT)
    let user = check_user_permission(&state, &headers, "contacts:update").await?;

    tracing::info!(
        "Patching contact: {} by user: {} (permission verified via middleware)",
        contact_id,
        user.id
    );

    let contact = ContactService::patch_contact(&state.db, contact_id, request, user.id).await?;

    let response = json!({
        "success": true,
        "message": "Contact patched successfully",
        "data": contact
    });

    Ok(Json(response))
}

/// Delete a contact by ID with permission checking (soft delete)
/// DELETE /api/contacts/:id
pub async fn delete_contact(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(contact_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Permission: delete contacts
    let user = check_user_permission(&state, &headers, "contacts:delete").await?;

    ContactService::delete_contact(&state.db, contact_id, user.id).await?;

    Ok(StatusCode::NO_CONTENT)
}


