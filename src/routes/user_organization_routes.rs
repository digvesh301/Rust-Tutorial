use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;

use crate::controllers::{
    add_user_to_organization,
    invite_user_to_organization,
    update_user_organization,
    remove_user_from_organization
};

/// Create user-organization relationship routes (protected - require authentication)
pub fn user_organization_routes() -> Router<PgPool> {
    Router::new()
        // User-Organization relationship management
        .route("/user-organizations", post(add_user_to_organization))
        .route("/user-organizations/invite", post(invite_user_to_organization))
        .route("/user-organizations/:id", put(update_user_organization))
        .route("/user-organizations/:id", delete(remove_user_from_organization))
        
        // Future user-organization routes
        // .route("/user-organizations", get(get_user_organizations_list))
        // .route("/user-organizations/:id", get(get_user_organization_by_id))
        // .route("/user-organizations/bulk-invite", post(bulk_invite_users))
        // .route("/user-organizations/accept-invitation/:token", post(accept_invitation))
        // .route("/user-organizations/decline-invitation/:token", post(decline_invitation))
}

/// Create public user-organization routes (no authentication required)
pub fn public_user_organization_routes() -> Router<PgPool> {
    Router::new()
        // Public invitation handling
        // .route("/invitations/:token", get(get_invitation_info))
        // .route("/invitations/:token/accept", post(accept_public_invitation))
        // .route("/invitations/:token/decline", post(decline_public_invitation))
}

/// User-Organization route configuration
pub struct UserOrganizationRouteConfig {
    pub enable_user_management: bool,
    pub enable_invitations: bool,
    pub enable_bulk_operations: bool,
    pub enable_public_invitations: bool,
}

impl Default for UserOrganizationRouteConfig {
    fn default() -> Self {
        Self {
            enable_user_management: true,
            enable_invitations: true,
            enable_bulk_operations: false,
            enable_public_invitations: false,
        }
    }
}

/// Create user-organization routes with configuration
pub fn user_organization_routes_with_config(config: UserOrganizationRouteConfig) -> Router<PgPool> {
    let mut router = Router::new();

    if config.enable_user_management {
        router = router
            .route("/user-organizations", post(add_user_to_organization))
            .route("/user-organizations/:id", put(update_user_organization))
            .route("/user-organizations/:id", delete(remove_user_from_organization));
    }

    if config.enable_invitations {
        router = router.route("/user-organizations/invite", post(invite_user_to_organization));
    }

    if config.enable_public_invitations {
        router = router.merge(public_user_organization_routes());
    }

    // Future bulk operations
    // if config.enable_bulk_operations {
    //     router = router.route("/user-organizations/bulk-invite", post(bulk_invite_users));
    // }

    router
}
