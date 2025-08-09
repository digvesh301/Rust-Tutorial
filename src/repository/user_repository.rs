// User Repository - Database operations for users

use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::User;

pub struct UserRepository;

impl UserRepository {
    /// Create users table if it doesn't exist
    pub async fn create_table(pool: &PgPool) -> Result<(), AppError> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                password VARCHAR(255) NOT NULL,
                status VARCHAR(50) NOT NULL DEFAULT 'active',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
        "#;

        sqlx::query(query).execute(pool).await?;

        // Create indexes
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)",
            "CREATE INDEX IF NOT EXISTS idx_users_status ON users(status)",
            "CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at)",
        ];

        for index_query in indexes {
            sqlx::query(index_query).execute(pool).await?;
        }
        
        tracing::info!("Users table created successfully");
        
        Ok(())
    }

    /// Insert a new user into the database
    pub async fn create(
        pool: &PgPool,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, AppError> {
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, email, password, status)
            VALUES ($1, $2, $3, 'active')
            RETURNING id, name, email, password, status, created_at, updated_at
            "#,
        )
        .bind(name.trim())
        .bind(email.trim().to_lowercase())
        .bind(password)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// Find user by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
        let result = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, password, status, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Find user by email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
        let result = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, password, status, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email.trim().to_lowercase())
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Get all users with pagination
    pub async fn find_all(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>, AppError> {
        let results = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, password, status, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    /// Update user status
    pub async fn update_status(
        pool: &PgPool,
        id: Uuid,
        status: &str,
    ) -> Result<User, AppError> {
        let result = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, name, email, password, status, created_at, updated_at
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// Check if email exists
    pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as exists
            "#,
        )
        .bind(email.trim().to_lowercase())
        .fetch_one(pool)
        .await?;

        let exists: bool = result.get("exists");
        Ok(exists)
    }

    /// Update user password
    pub async fn update_password(
        pool: &PgPool,
        id: Uuid,
        hashed_password: &str,
    ) -> Result<User, AppError> {
        let result = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET password = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, name, email, password, status, created_at, updated_at
            "#,
        )
        .bind(hashed_password)
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }
}
