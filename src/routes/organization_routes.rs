use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;

use crate::controllers::{
    create_organization,
    get_organization_users
};

/// Create organization-related routes (protected - require authentication)
pub fn organization_routes() -> Router<PgPool> {
    Router::new()
        // Organization management
        .route("/organizations", post(create_organization))
        .route("/organizations/:org_id/users", get(get_organization_users))
        
        // Future organization routes
        // .route("/organizations", get(get_organizations))
        // .route("/organizations/:id", get(get_organization_by_id))
        // .route("/organizations/:id", put(update_organization))
        // .route("/organizations/:id", delete(delete_organization))
        // .route("/organizations/:id/settings", get(get_organization_settings))
        // .route("/organizations/:id/settings", put(update_organization_settings))
}

/// Create public organization routes (no authentication required)
pub fn public_organization_routes() -> Router<PgPool> {
    Router::new()
        // Currently no public organization routes
        // Future: public organization info, signup pages, etc.
        // .route("/organizations/public/:slug", get(get_public_organization_info))
}

/// Organization route configuration
pub struct OrganizationRouteConfig {
    pub enable_organization_creation: bool,
    pub enable_organization_management: bool,
    pub enable_public_info: bool,
}

impl Default for OrganizationRouteConfig {
    fn default() -> Self {
        Self {
            enable_organization_creation: true,
            enable_organization_management: true,
            enable_public_info: false,
        }
    }
}

/// Create organization routes with configuration
pub fn organization_routes_with_config(config: OrganizationRouteConfig) -> Router<PgPool> {
    let mut router = Router::new();

    if config.enable_organization_creation {
        router = router.route("/organizations", post(create_organization));
    }

    if config.enable_organization_management {
        router = router.route("/organizations/:org_id/users", get(get_organization_users));
    }

    if config.enable_public_info {
        router = router.merge(public_organization_routes());
    }

    router
}
