// Organization Repository - Database operations for organizations

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::Organization;

pub struct OrganizationRepository;

impl OrganizationRepository {
    /// Create organization table if it doesn't exist
    pub async fn create_table(pool: &PgPool) -> Result<(), AppError> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS organization (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(255) NOT NULL,
                country VARCHAR(100),
                timezone VARCHAR(50),
                created_at TIMESTAMPTZ DEFAULT NOW()
            )
        "#;

        sqlx::query(query).execute(pool).await?;
        
        tracing::info!("Organization table created successfully");
        
        Ok(())
    }

    /// Insert a new organization into the database
    pub async fn create(pool: &PgPool, name: String, country: Option<String>, timezone: Option<String>) -> Result<Organization, AppError> {
        let result = sqlx::query_as!(
            Organization,
            r#"
            INSERT INTO organization (name, country, timezone)
            VALUES ($1, $2, $3)
            RETURNING id, name, country, timezone, created_at
            "#,
            name.trim(),
            country,
            timezone
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// Find organization by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Organization>, AppError> {
        let result = sqlx::query_as!(
            Organization,
            r#"
            SELECT id, name, country, timezone, created_at
            FROM organization
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Get all organizations
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Organization>, AppError> {
        let results = sqlx::query_as!(
            Organization,
            r#"
            SELECT id, name, country, timezone, created_at
            FROM organization
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }
}
