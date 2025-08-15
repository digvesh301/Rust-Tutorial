use axum::{
    routing::{get, post},
    Router,
};

use crate::controllers::custom_field_controller::{create_custom_field, get_custom_fields_by_module};
use crate::AppState;

pub fn custom_field_routes() -> Router<AppState> {
    Router::new()
        .route("/custom-fields", post(create_custom_field))
        .route("/custom-fields/:module", get(get_custom_fields_by_module))
}