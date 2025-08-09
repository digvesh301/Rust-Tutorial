// Database connection module for CockroachDB

pub mod migrations;

use sqlx::PgPool;
use std::env;

pub use migrations::*;

/// Creates a connection pool to CockroachDB
///
/// This function establishes a connection to CockroachDB using the DATABASE_URL
/// environment variable. It returns a connection pool that can be shared across
/// the application.
pub async fn create_connection_pool() -> Result<PgPool, sqlx::Error> {
    // Get the database URL from environment variable
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment variables");

    // Create connection pool
    let pool = PgPool::connect(&database_url).await?;

    tracing::info!("Successfully connected to CockroachDB");

    Ok(pool)
}

/// Creates the organization table if it doesn't exist
///
/// This function creates the organization table using the repository layer
pub async fn create_organization_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    use crate::repository::OrganizationRepository;

    OrganizationRepository::create_table(pool)
        .await
        .map_err(|e| match e {
            crate::errors::AppError::DatabaseError(db_err) => db_err,
            _ => sqlx::Error::Protocol("Failed to create organization table".to_string()),
        })
}
