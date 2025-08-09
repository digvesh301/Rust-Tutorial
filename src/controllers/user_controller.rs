// User Controller - Handles user-related HTTP requests

use axum::{
    extract::{Path, Query, Request, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{CreateUserRequest, LoginRequest, LoginResponse, UpdatePasswordRequest, UpdateUserStatusRequest, UserCreationResponse, UserResponse};
use crate::errors::AppError;
use crate::middleware::extract_user_from_request;
use crate::services::UserService;

#[derive(Debug, Deserialize)]
pub struct UserQueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<String>,
}

/// POST /users - Create a new user
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserCreationResponse>), AppError> {
    let response = UserService::create_user(&pool, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// GET /users - Get all users with optional pagination and filtering
pub async fn get_users(
    State(pool): State<PgPool>,
    Query(params): Query<UserQueryParams>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    
    let users = UserService::get_all_users(&pool, page, limit, params.status).await?;
    Ok(Json(users))
}

/// GET /users/:id - Get user by ID
pub async fn get_user_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = UserService::get_user_by_id(&pool, id).await?;
    
    match user {
        Some(user_response) => Ok(Json(user_response)),
        None => Err(AppError::NotFound(format!("User with id {} not found", id))),
    }
}

/// PUT /users/:id - Update user status
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserStatusRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let response = UserService::update_user_status(&pool, id, payload.status).await?;
    Ok(Json(response))
}

/// DELETE /users/:id - Delete user (soft delete by setting status to 'inactive')
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    UserService::delete_user(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /users/login - Login user with email and password
pub async fn login_user(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let response = UserService::login_user(&pool, payload).await?;
    Ok(Json(response))
}

/// PUT /users/:id/password - Update user password
pub async fn update_user_password(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePasswordRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let response = UserService::update_password(&pool, id, payload).await?;
    Ok(Json(response))
}

/// GET /users/me - Get current authenticated user (Protected route)
pub async fn get_current_user(
    request: Request,
) -> Result<Json<UserResponse>, AppError> {
    // Extract user from JWT token (injected by middleware)
    let jwt_user = extract_user_from_request(&request)?;

    // Convert JWT user to response
    let response = UserResponse {
        id: jwt_user.id,
        name: jwt_user.name.clone(),
        email: jwt_user.email.clone(),
        status: jwt_user.status.clone(),
        created_at: "N/A".to_string(), // JWT doesn't contain timestamps
        updated_at: "N/A".to_string(),
    };

    Ok(Json(response))
}
