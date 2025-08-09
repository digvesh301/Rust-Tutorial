// Password utility functions for hashing and verification

use bcrypt::{hash, verify, DEFAULT_COST};
use crate::errors::AppError;

/// Hash a plain text password using bcrypt
pub fn hash_password(password: &str) -> Result<String, AppError> {
    // Validate password strength
    if password.len() < 6 {
        return Err(AppError::ValidationError(
            "Password must be at least 6 characters long".to_string(),
        ));
    }

    if password.len() > 72 {
        return Err(AppError::ValidationError(
            "Password cannot exceed 72 characters (bcrypt limitation)".to_string(),
        ));
    }

    // Hash the password with default cost (12)
    hash(password, DEFAULT_COST).map_err(|e| {
        tracing::error!("Failed to hash password: {}", e);
        AppError::InternalServerError("Failed to hash password".to_string())
    })
}

/// Verify a plain text password against a hashed password
pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    verify(password, hashed_password).map_err(|e| {
        tracing::error!("Failed to verify password: {}", e);
        AppError::InternalServerError("Failed to verify password".to_string())
    })
}

/// Generate a secure random password (for testing or password reset)
pub fn generate_random_password(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@#$%^&*";
    
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "test_password_123";
        let hashed = hash_password(password).unwrap();
        
        // Verify the password matches
        assert!(verify_password(password, &hashed).unwrap());
        
        // Verify wrong password doesn't match
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }

    #[test]
    fn test_password_validation() {
        // Too short password
        assert!(hash_password("12345").is_err());
        
        // Too long password (73 characters)
        let long_password = "a".repeat(73);
        assert!(hash_password(&long_password).is_err());
        
        // Valid password
        assert!(hash_password("valid_password_123").is_ok());
    }

    #[test]
    fn test_random_password_generation() {
        let password = generate_random_password(12);
        assert_eq!(password.len(), 12);
        
        // Test that generated passwords are different
        let password2 = generate_random_password(12);
        assert_ne!(password, password2);
    }
}
