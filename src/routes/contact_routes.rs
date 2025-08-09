use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use crate::AppState;

use crate::controllers::{create_contact, get_contact};

/// Create contact routes with permissions (for AppState)
pub fn contact_routes_with_permissions() -> Router<AppState> {
    Router::new()
        // Create contact
        .route("/contacts", post(create_contact))
        // View single contact
        .route("/contacts/:id", get(get_contact))
}

/// Create empty contact routes for PgPool compatibility
pub fn contact_routes() -> Router<PgPool> {
    Router::new()
        // No routes - all functionality moved to permission-based routes
}
