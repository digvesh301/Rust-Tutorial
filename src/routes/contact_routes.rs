use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};
use sqlx::PgPool;
use crate::AppState;

use crate::controllers::{create_contact, get_contact, update_contact, patch_contact, delete_contact};

/// Create contact routes with permissions (for AppState)
pub fn contact_routes_with_permissions() -> Router<AppState> {
    Router::new()
        // Create contact
        .route("/contacts", post(create_contact))
        // View single contact
        .route("/contacts/:id", get(get_contact))
        // Update contact (full replacement)
        .route("/contacts/:id", put(update_contact))
        // Patch contact (partial update with merge semantics)
        .route("/contacts/:id", patch(patch_contact))
        // Delete contact
        .route("/contacts/:id", delete(delete_contact))
}

/// Create empty contact routes for PgPool compatibility
pub fn contact_routes() -> Router<PgPool> {
    Router::new()
        // No routes - all functionality moved to permission-based routes
}
