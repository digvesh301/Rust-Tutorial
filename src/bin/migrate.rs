// Standalone migration runner

use survey::database::{create_connection_pool, MigrationRunner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .compact()
        .init();

    println!("🚀 Starting database migrations...");

    // Create database connection
    let pool = create_connection_pool()
        .await
        .expect("Failed to create database connection pool");

    // Run migrations
    MigrationRunner::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    println!("✅ All migrations completed successfully!");

    Ok(())
}
