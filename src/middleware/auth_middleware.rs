// JWT Authentication Middleware

use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::utils::{extract_token_from_header, validate_token, JwtUser};

/// JWT Authentication middleware
/// Validates JWT token and injects user info into request extensions
pub async fn jwt_auth_middleware(
    State(_pool): State<PgPool>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            AppError::ValidationError("Missing Authorization header".to_string())
        })?;

    // Extract token from header
    let token = extract_token_from_header(auth_header)?;

    // Validate token and extract claims
    let claims = validate_token(token)?;

    // Convert claims to user info
    let user = claims.to_user()?;

    // Check if user is active
    if user.status != "active" {
        return Err(AppError::ValidationError(
            format!("User account is {}", user.status),
        ));
    }

    // Insert user info into request extensions
    request.extensions_mut().insert(user);

    // Continue to the next middleware/handler
    Ok(next.run(request).await)
}

/// Optional JWT Authentication middleware
/// Similar to jwt_auth_middleware but doesn't fail if no token is provided
/// Useful for routes that can work with or without authentication
pub async fn optional_jwt_auth_middleware(
    State(_pool): State<PgPool>,
    mut request: Request,
    next: Next,
) -> Response {
    // Try to extract Authorization header
    if let Some(auth_header) = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
    {
        // Try to extract and validate token
        if let Ok(token) = extract_token_from_header(auth_header) {
            if let Ok(claims) = validate_token(token) {
                if let Ok(user) = claims.to_user() {
                    if user.status == "active" {
                        // Insert user info into request extensions
                        request.extensions_mut().insert(user);
                    }
                }
            }
        }
    }

    // Continue to the next middleware/handler regardless of auth status
    next.run(request).await
}

/// Extract authenticated user from request extensions
/// This function should be used in handlers that are protected by jwt_auth_middleware
pub fn extract_user_from_request(request: &Request) -> Result<&JwtUser, AppError> {
    request
        .extensions()
        .get::<JwtUser>()
        .ok_or_else(|| {
            AppError::InternalServerError("User not found in request extensions".to_string())
        })
}

/// Extract optional authenticated user from request extensions
/// This function should be used in handlers that use optional_jwt_auth_middleware
pub fn extract_optional_user_from_request(request: &Request) -> Option<&JwtUser> {
    request.extensions().get::<JwtUser>()
}
