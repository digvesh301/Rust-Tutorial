// JWT utility functions for token generation and validation

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub email: String,      // User email
    pub name: String,       // User name
    pub status: String,     // User status
    pub exp: i64,          // Expiration time
    pub iat: i64,          // Issued at
}

#[derive(Debug, Clone)]
pub struct JwtUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub status: String,
}

impl Claims {
    /// Create new claims for a user
    pub fn new(user_id: Uuid, email: String, name: String, status: String) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours

        Self {
            sub: user_id.to_string(),
            email,
            name,
            status,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }

    /// Convert claims to JwtUser
    pub fn to_user(&self) -> Result<JwtUser, AppError> {
        let id = Uuid::parse_str(&self.sub).map_err(|_| {
            AppError::ValidationError("Invalid user ID in token".to_string())
        })?;

        Ok(JwtUser {
            id,
            email: self.email.clone(),
            name: self.name.clone(),
            status: self.status.clone(),
        })
    }
}

/// Get JWT secret from environment or use default
fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set, using default secret. This is not secure for production!");
        "your-secret-key-change-this-in-production".to_string()
    })
}

/// Generate JWT token for a user
pub fn generate_token(user_id: Uuid, email: String, name: String, status: String) -> Result<String, AppError> {
    let claims = Claims::new(user_id, email, name, status);
    let secret = get_jwt_secret();
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| {
        tracing::error!("Failed to generate JWT token: {}", e);
        AppError::InternalServerError("Failed to generate authentication token".to_string())
    })
}

/// Validate JWT token and extract claims
pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    let secret = get_jwt_secret();
    let validation = Validation::new(Algorithm::HS256);
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| {
        tracing::debug!("JWT validation failed: {}", e);
        AppError::ValidationError("Invalid or expired authentication token".to_string())
    })
}

/// Extract token from Authorization header
pub fn extract_token_from_header(auth_header: &str) -> Result<&str, AppError> {
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::ValidationError(
            "Authorization header must start with 'Bearer '".to_string(),
        ));
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix
    if token.is_empty() {
        return Err(AppError::ValidationError(
            "Authorization token is empty".to_string(),
        ));
    }

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_validation() {
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let name = "Test User".to_string();
        let status = "active".to_string();

        // Generate token
        let token = generate_token(user_id, email.clone(), name.clone(), status.clone()).unwrap();
        assert!(!token.is_empty());

        // Validate token
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.name, name);
        assert_eq!(claims.status, status);

        // Convert to user
        let jwt_user = claims.to_user().unwrap();
        assert_eq!(jwt_user.id, user_id);
        assert_eq!(jwt_user.email, email);
    }

    #[test]
    fn test_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_token_from_header() {
        // Valid header
        let header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
        let token = extract_token_from_header(header).unwrap();
        assert_eq!(token, "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...");

        // Invalid header
        let invalid_header = "InvalidHeader token";
        assert!(extract_token_from_header(invalid_header).is_err());

        // Empty token
        let empty_header = "Bearer ";
        assert!(extract_token_from_header(empty_header).is_err());
    }
}
