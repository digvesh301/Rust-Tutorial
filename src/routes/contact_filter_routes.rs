use axum::{
    routing::{get, post},
    Router,
};

use crate::controllers::contact_filter_controller::{
    filter_contacts, get_filter_fields, get_filter_presets, validate_filter,
};
use crate::AppState;

/// Create contact filter routes with permissions (for AppState)
pub fn contact_filter_routes_with_permissions() -> Router<AppState> {
    Router::new()
        // Main filter endpoint
        .route("/contacts/filter", post(filter_contacts))
        // Get available filter fields and their types
        .route("/contacts/filter/fields", get(get_filter_fields))
        // Get filter presets/templates
        .route("/contacts/filter/presets", get(get_filter_presets))
        // Validate filter structure
        .route("/contacts/filter/validate", post(validate_filter))
}
