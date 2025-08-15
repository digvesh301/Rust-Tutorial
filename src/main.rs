// Survey Application - Main entry point

mod controllers;
mod database;
mod dto;
mod errors;
mod middleware;
mod models;
mod repository;
mod services;
mod utils;

use axum::{
    middleware::from_fn_with_state,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, Level};

use survey::routes::{
    contact_routes, contact_routes_with_permissions,
    contact_filter_routes::contact_filter_routes_with_permissions,
    user_routes, public_user_routes,
    organization_routes,
    user_organization_routes,
};
use crate::middleware::jwt_auth_middleware;
use crate::database::{create_connection_pool, create_organization_table, MigrationRunner};
use survey::AppState;



#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .compact()
        .init();

    info!("Survey Application Starting...");

    // Initialize database connection pool
    let db_pool = create_connection_pool()
        .await
        .expect("Failed to create database connection pool");

    info!("Database connection pool created successfully");

    // // Create organization table
    create_organization_table(&db_pool)
        .await
        .expect("Failed to create organization table");

    // Run database migrations
    MigrationRunner::run_migrations(&db_pool)
        .await
        .expect("Failed to run database migrations");

    info!("Database tables and migrations completed successfully");

    // Create application state
    let app_state = AppState {
        db: db_pool.clone(),
    };

    // Build our application with routes
    let app = create_app(app_state);

    // Define the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    info!("Server listening on {}", addr);

    // Create a TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Run the server
    axum::serve(listener, app).await.unwrap();
}

fn create_app(app_state: AppState) -> Router {
    // Public routes (no authentication required)	
    let public_routes = Router::new()
        // Root route
        .route("/", get(root))
        // Health check route
        .route("/health", get(health_check))
        // Merge public user routes (registration, login)
        .merge(public_user_routes());

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        // Merge all protected route modules
        .merge(user_routes())
        .merge(organization_routes())
        .merge(user_organization_routes())
        .merge(contact_routes())
        // Add JWT authentication middleware to all protected routes
        .layer(from_fn_with_state(
            app_state.db.clone(),
            jwt_auth_middleware,
        ));

    // Permission-protected routes (authentication + permission required)
    let permission_protected_routes = Router::new()
        .merge(contact_routes_with_permissions())
        .merge(contact_filter_routes_with_permissions())
        .layer(from_fn_with_state(
            app_state.db.clone(),
            jwt_auth_middleware,
        ))
        .with_state(app_state.clone());



    // Combine routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(permission_protected_routes)
        .with_state(app_state.db)
        // Add global middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
}

// Handler functions
async fn root() -> Json<Value> {
    Json(json!({
        "message": "Welcome to Survey API",
        "version": "1.0.0",
        "status": "running"
    }))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Server is healthy"
    }))
}


