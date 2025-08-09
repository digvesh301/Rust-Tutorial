// UserOrganization Controller - HTTP handlers for user-organization relationships

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{
    CreateUserOrganizationRequest, InviteUserToOrganizationRequest,
    UpdateUserOrganizationRequest, UserOrganizationDetailResponse,
    UserOrganizationQueryParams,
};
use crate::errors::AppError;
use crate::services::UserOrganizationService;

/// POST /user-organizations - Add user to organization
pub async fn add_user_to_organization(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserOrganizationRequest>,
) -> Result<(StatusCode, Json<UserOrganizationDetailResponse>), AppError> {
    // TODO: Add authentication check here when needed
    
    let response = UserOrganizationService::add_user_to_organization(&pool, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// POST /user-organizations/invite - Invite user to organization
pub async fn invite_user_to_organization(
    State(pool): State<PgPool>,
    Json(payload): Json<InviteUserToOrganizationRequest>,
) -> Result<(StatusCode, Json<UserOrganizationDetailResponse>), AppError> {
    // TODO: Add authentication check here when needed
    
    let response = UserOrganizationService::invite_user_to_organization(&pool, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// GET /users/:user_id/organizations - Get organizations for a user
pub async fn get_user_organizations(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<UserOrganizationQueryParams>,
) -> Result<Json<Vec<UserOrganizationDetailResponse>>, AppError> {
    // TODO: Add authentication and authorization checks here
    
    let response = UserOrganizationService::get_user_organizations(
        &pool, 
        user_id, 
        params.status
    ).await?;
    
    Ok(Json(response))
}

/// GET /organizations/:org_id/users - Get users for an organization
pub async fn get_organization_users(
    State(pool): State<PgPool>,
    Path(org_id): Path<Uuid>,
    Query(params): Query<UserOrganizationQueryParams>,
) -> Result<Json<Vec<UserOrganizationDetailResponse>>, AppError> {
    // TODO: Add authentication and authorization checks here
    
    let response = UserOrganizationService::get_organization_users(
        &pool, 
        org_id, 
        params.status
    ).await?;
    
    Ok(Json(response))
}

/// PUT /user-organizations/:id - Update user organization relationship
pub async fn update_user_organization(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserOrganizationRequest>,
) -> Result<Json<UserOrganizationDetailResponse>, AppError> {
    // TODO: Add authentication and authorization checks here
    
    let response = UserOrganizationService::update_user_organization(&pool, id, payload).await?;
    Ok(Json(response))
}

/// DELETE /user-organizations/:id - Remove user from organization
pub async fn remove_user_from_organization(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // TODO: Add authentication and authorization checks here
    
    UserOrganizationService::remove_user_from_organization(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /users/me/organizations - Get current user's organizations
pub async fn get_current_user_organizations(
    State(pool): State<PgPool>,
    Query(params): Query<UserOrganizationQueryParams>,
) -> Result<Json<Vec<UserOrganizationDetailResponse>>, AppError> {
    // TODO: Extract authenticated user from JWT middleware
    // For now, we'll use a dummy user ID
    let user_id = uuid::Uuid::new_v4(); // This should come from JWT
    
    let response = UserOrganizationService::get_user_organizations(
        &pool,
        user_id,
        params.status
    ).await?;
    
    Ok(Json(response))
}
