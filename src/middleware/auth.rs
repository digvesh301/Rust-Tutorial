use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::models::User;
use crate::AppState;

/// Authenticated user extracted from JWT token
#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub User);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract user from request extensions (set by JWT middleware)
        if let Some(user) = parts.extensions.get::<User>() {
            Ok(AuthenticatedUser(user.clone()))
        } else {
            let error_response = Json(json!({
                "error": "Unauthorized",
                "message": "Authentication required"
            }));
            Err((StatusCode::UNAUTHORIZED, error_response).into_response())
        }
    }
}
