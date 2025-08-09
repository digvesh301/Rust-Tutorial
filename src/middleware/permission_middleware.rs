// Permission middleware for role-based access control

use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    services::PermissionService,
    utils::jwt_utils::{validate_token, JwtUser},
    AppState,
};

/// Simple permission checking function that accepts headers directly
pub async fn check_user_permission(
    state: &AppState,
    headers: &axum::http::HeaderMap,
    required_permission: &str,
) -> Result<JwtUser, AppError> {
    // Extract JWT token from headers
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization header format".to_string()))?;

    // Validate JWT token and extract claims
    let claims = validate_token(token)?;

    // Convert claims to JwtUser
    let jwt_user = JwtUser {
        id: Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?,
        email: claims.email,
        name: claims.name,
        status: claims.status,
    };

    // Get user's organization
    let org_id = get_user_organization(state, jwt_user.id).await?;

    // Check if user has the required permission
    let has_permission = PermissionService::has_permission(
        &state.db,
        jwt_user.id,
        org_id,
        required_permission,
    ).await?;

    if !has_permission {
        return Err(AppError::Unauthorized(format!(
            "Permission '{}' required",
            required_permission
        )));
    }

    Ok(jwt_user)
}

/// Permission checking function that accepts token directly (for cases where token is already extracted)
pub async fn check_user_permission_with_token(
    state: &AppState,
    token: &str,
    required_permission: &str,
) -> Result<JwtUser, AppError> {
    // Validate JWT token and extract claims
    let claims = validate_token(token)?;

    // Convert claims to JwtUser
    let jwt_user = JwtUser {
        id: Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?,
        email: claims.email,
        name: claims.name,
        status: claims.status,
    };

    // Get user's organization
    let org_id = get_user_organization(state, jwt_user.id).await?;

    // Check if user has the required permission
    let has_permission = PermissionService::has_permission(
        &state.db,
        jwt_user.id,
        org_id,
        required_permission,
    ).await?;

    if !has_permission {
        return Err(AppError::Unauthorized(format!(
            "Permission '{}' required",
            required_permission
        )));
    }

    Ok(jwt_user)
}

/// Check if user has any of the required permissions
pub async fn check_any_permission(
    state: &AppState,
    headers: &axum::http::HeaderMap,
    required_permissions: &[&str],
) -> Result<JwtUser, AppError> {
    // Extract JWT token from headers
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization header format".to_string()))?;

    let claims = validate_token(token)?;
    let jwt_user = JwtUser {
        id: Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?,
        email: claims.email,
        name: claims.name,
        status: claims.status,
    };

    let org_id = get_user_organization(state, jwt_user.id).await?;

    let has_any_permission = PermissionService::has_any_permission(
        &state.db,
        jwt_user.id,
        org_id,
        required_permissions,
    ).await?;

    if !has_any_permission {
        return Err(AppError::Unauthorized(format!(
            "One of these permissions required: {}",
            required_permissions.join(", ")
        )));
    }

    Ok(jwt_user)
}

/// Check if user has all of the required permissions
pub async fn check_all_permissions(
    state: &AppState,
    token: &str,
    required_permissions: &[&str],
) -> Result<JwtUser, AppError> {
    let claims = validate_token(token)?;
    let jwt_user = JwtUser {
        id: Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?,
        email: claims.email,
        name: claims.name,
        status: claims.status,
    };

    let org_id = get_user_organization(state, jwt_user.id).await?;

    let has_all_permissions = PermissionService::has_all_permissions(
        &state.db,
        jwt_user.id,
        org_id,
        required_permissions,
    ).await?;

    if !has_all_permissions {
        return Err(AppError::Unauthorized(format!(
            "All of these permissions required: {}",
            required_permissions.join(", ")
        )));
    }

    Ok(jwt_user)
}

/// Check resource ownership permissions
pub async fn check_resource_ownership(
    state: &AppState,
    token: &str,
    base_permission: &str,
    resource_owner_id: Option<Uuid>,
) -> Result<JwtUser, AppError> {
    let claims = validate_token(token)?;
    let jwt_user = JwtUser {
        id: Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?,
        email: claims.email,
        name: claims.name,
        status: claims.status,
    };

    let org_id = get_user_organization(state, jwt_user.id).await?;

    // Check if user has full permission
    let has_permission = PermissionService::has_permission(
        &state.db,
        jwt_user.id,
        org_id,
        base_permission,
    ).await?;

    if has_permission {
        return Ok(jwt_user);
    }

    // Check if user has "own" permission and owns the resource
    let own_permission = format!("{}:own", base_permission);
    let has_own_permission = PermissionService::has_permission(
        &state.db,
        jwt_user.id,
        org_id,
        &own_permission,
    ).await?;

    if has_own_permission {
        if let Some(owner_id) = resource_owner_id {
            if owner_id == jwt_user.id {
                return Ok(jwt_user);
            }
        }
    }

    Err(AppError::Unauthorized(format!(
        "Permission '{}' or ownership required",
        base_permission
    )))
}

/// Helper function to get user's organization
async fn get_user_organization(state: &AppState, user_id: Uuid) -> Result<Uuid, AppError> {
    let query = r#"
        SELECT org_id
        FROM user_organizations
        WHERE user_id = $1 AND status = 'active'
        LIMIT 1
    "#;

    let row = sqlx::query_as::<_, (Uuid,)>(query)
        .bind(user_id)
        .fetch_optional(&state.db)
        .await?;

    match row {
        Some((org_id,)) => Ok(org_id),
        None => Err(AppError::Unauthorized("User not associated with any organization".to_string())),
    }
}

/// Extract token from request headers
pub fn extract_token_from_request(request: &Request) -> Result<&str, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("Invalid authorization header format".to_string()));
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix
    if token.is_empty() {
        return Err(AppError::Unauthorized("Empty authorization token".to_string()));
    }

    Ok(token)
}
