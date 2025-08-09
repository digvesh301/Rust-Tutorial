// User Service - Business logic for users

use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{CreateUserRequest, LoginRequest, LoginResponse, UpdatePasswordRequest, UserCreationResponse, UserResponse};
use crate::errors::AppError;
use crate::models::User;
use crate::repository::UserRepository;
use crate::utils::{format_timestamp, generate_token, hash_password, verify_password};

pub struct UserService;

impl UserService {
    /// Create a new user with validation
    pub async fn create_user(
        pool: &PgPool,
        request: CreateUserRequest,
    ) -> Result<UserCreationResponse, AppError> {
        // Validate required fields
        if request.name.trim().is_empty() {
            return Err(AppError::ValidationError(
                "User name is required and cannot be empty".to_string(),
            ));
        }

        if request.email.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Email is required and cannot be empty".to_string(),
            ));
        }

        if request.password.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Password is required and cannot be empty".to_string(),
            ));
        }

        // Validate name length
        if request.name.trim().len() > 255 {
            return Err(AppError::ValidationError(
                "User name cannot exceed 255 characters".to_string(),
            ));
        }

        // Validate email format (basic validation)
        if !request.email.contains('@') || !request.email.contains('.') {
            return Err(AppError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        // Check if email already exists
        if UserRepository::email_exists(pool, &request.email).await? {
            return Err(AppError::ValidationError(
                "Email already exists".to_string(),
            ));
        }

        // Hash password before storing
        let hashed_password = hash_password(&request.password)?;

        // Create user using repository
        let user = UserRepository::create(
            pool,
            request.name.trim().to_string(),
            request.email.trim().to_lowercase(),
            hashed_password,
        )
        .await?;

        // Generate JWT token
        let token = generate_token(user.id, user.email.clone(), user.name.clone(), user.status.clone())?;

        // Convert to response DTO with token
        Ok(UserCreationResponse {
            user: Self::to_response(user),
            token,
        })
    }

    /// Get user by ID
    pub async fn get_user_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<UserResponse>, AppError> {
        let user = UserRepository::find_by_id(pool, id).await?;
        
        Ok(user.map(Self::to_response))
    }

    /// Get all users with pagination and status filter
    pub async fn get_all_users(
        pool: &PgPool,
        page: u32,
        limit: u32,
        status_filter: Option<String>,
    ) -> Result<Vec<UserResponse>, AppError> {
        // Calculate offset
        let offset = ((page - 1) * limit) as i64;
        let limit = limit as i64;

        // For now, we'll use the basic find_all method
        // TODO: Implement status filtering in repository
        let users = UserRepository::find_all(pool, limit, offset).await?;
        
        // Filter by status if provided
        let filtered_users = if let Some(status) = status_filter {
            users.into_iter()
                .filter(|user| user.status == status)
                .collect()
        } else {
            users
        };
        
        Ok(filtered_users.into_iter().map(Self::to_response).collect())
    }

    /// Update user status
    pub async fn update_user_status(
        pool: &PgPool,
        id: Uuid,
        status: String,
    ) -> Result<UserResponse, AppError> {
        // Validate status
        let valid_statuses = ["active", "inactive", "suspended", "pending"];
        if !valid_statuses.contains(&status.as_str()) {
            return Err(AppError::ValidationError(
                format!("Invalid status. Must be one of: {}", valid_statuses.join(", "))
            ));
        }

        // Check if user exists
        let existing_user = UserRepository::find_by_id(pool, id).await?;
        if existing_user.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", id)));
        }

        // Update user status
        let user = UserRepository::update_status(pool, id, &status).await?;
        
        Ok(Self::to_response(user))
    }

    /// Delete user (soft delete by setting status to 'inactive')
    pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        // Check if user exists
        let existing_user = UserRepository::find_by_id(pool, id).await?;
        if existing_user.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", id)));
        }

        // Soft delete by setting status to 'inactive'
        UserRepository::update_status(pool, id, "inactive").await?;

        Ok(())
    }

    /// Login user with email and password
    pub async fn login_user(
        pool: &PgPool,
        request: LoginRequest,
    ) -> Result<LoginResponse, AppError> {
        // Validate required fields
        if request.email.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Email is required".to_string(),
            ));
        }

        if request.password.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Password is required".to_string(),
            ));
        }

        // Find user by email
        let user = UserRepository::find_by_email(pool, &request.email).await?;

        let user = match user {
            Some(user) => user,
            None => {
                return Err(AppError::ValidationError(
                    "Invalid email or password".to_string(),
                ));
            }
        };

        // Check if user is active
        if user.status != "active" {
            return Err(AppError::ValidationError(
                format!("User account is {}", user.status),
            ));
        }

        // Verify password
        let is_valid = verify_password(&request.password, &user.password)?;

        if !is_valid {
            return Err(AppError::ValidationError(
                "Invalid email or password".to_string(),
            ));
        }

        // Generate JWT token
        let token = generate_token(user.id, user.email.clone(), user.name.clone(), user.status.clone())?;

        // Create login response
        let response = LoginResponse {
            user: Self::to_response(user),
            token: Some(token),
        };

        Ok(response)
    }

    /// Update user password
    pub async fn update_password(
        pool: &PgPool,
        id: Uuid,
        request: UpdatePasswordRequest,
    ) -> Result<UserResponse, AppError> {
        // Find user by ID
        let existing_user = UserRepository::find_by_id(pool, id).await?;
        let user = match existing_user {
            Some(user) => user,
            None => {
                return Err(AppError::NotFound(format!("User with id {} not found", id)));
            }
        };

        // Verify current password
        let is_current_valid = verify_password(&request.current_password, &user.password)?;
        if !is_current_valid {
            return Err(AppError::ValidationError(
                "Current password is incorrect".to_string(),
            ));
        }

        // Hash new password
        let new_hashed_password = hash_password(&request.new_password)?;

        // Update password in database
        let updated_user = UserRepository::update_password(pool, id, &new_hashed_password).await?;

        Ok(Self::to_response(updated_user))
    }

    /// Convert User model to response DTO
    fn to_response(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            status: user.status,
            created_at: format_timestamp(user.created_at),
            updated_at: format_timestamp(user.updated_at),
        }
    }

    /// Convert User model to response DTO with JWT token
    fn to_response_with_token(user: User) -> UserResponse {
        // For now, we'll return the same response as to_response
        // In a real application, you might want a different response type for user creation
        // that includes the token, but for simplicity we'll use the existing UserResponse
        Self::to_response(user)
    }
}
