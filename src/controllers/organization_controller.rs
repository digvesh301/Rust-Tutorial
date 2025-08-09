// Organization Controller - Handles organization-related HTTP requests

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use sqlx::PgPool;

use crate::dto::{CreateOrganizationRequest, OrganizationResponse};
use crate::errors::AppError;
use crate::services::OrganizationService;

/// Create a new organization
pub async fn create_organization(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateOrganizationRequest>,
) -> Result<(StatusCode, Json<OrganizationResponse>), AppError> {
    let response = OrganizationService::create_organization(&pool, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
