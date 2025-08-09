use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;

use crate::controllers::{
    create_user, 
    get_users, 
    get_user_by_id, 
    update_user, 
    delete_user, 
    login_user, 
    update_user_password, 
    get_current_user,
    get_current_user_organizations,
    get_user_organizations
};

/// Create user-related routes (protected - require authentication)
pub fn user_routes() -> Router<PgPool> {
    Router::new()
        // User management
        .route("/users", get(get_users))
        .route("/users/me", get(get_current_user))
        .route("/users/me/organizations", get(get_current_user_organizations))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .route("/users/:id/password", put(update_user_password))
        .route("/users/:user_id/organizations", get(get_user_organizations))
}

/// Create public user routes (no authentication required)
pub fn public_user_routes() -> Router<PgPool> {
    Router::new()
        // Authentication routes
        .route("/users", post(create_user))
        .route("/users/login", post(login_user))
        // Add other public routes like password reset, email verification, etc.
        // .route("/users/forgot-password", post(forgot_password))
        // .route("/users/reset-password", post(reset_password))
        // .route("/users/verify-email", post(verify_email))
}

/// User route configuration
pub struct UserRouteConfig {
    pub enable_registration: bool,
    pub enable_password_reset: bool,
    pub enable_email_verification: bool,
    pub enable_user_management: bool,
}

impl Default for UserRouteConfig {
    fn default() -> Self {
        Self {
            enable_registration: true,
            enable_password_reset: false,
            enable_email_verification: false,
            enable_user_management: true,
        }
    }
}

/// Create user routes with configuration
pub fn user_routes_with_config(config: UserRouteConfig) -> (Router<PgPool>, Router<PgPool>) {
    // Protected routes
    let mut protected_routes = Router::new();
    
    if config.enable_user_management {
        protected_routes = protected_routes
            .route("/users", get(get_users))
            .route("/users/me", get(get_current_user))
            .route("/users/me/organizations", get(get_current_user_organizations))
            .route("/users/:id", get(get_user_by_id))
            .route("/users/:id", put(update_user))
            .route("/users/:id", delete(delete_user))
            .route("/users/:id/password", put(update_user_password))
            .route("/users/:user_id/organizations", get(get_user_organizations));
    }

    // Public routes
    let mut public_routes = Router::new()
        .route("/users/login", post(login_user));

    if config.enable_registration {
        public_routes = public_routes.route("/users", post(create_user));
    }

    // Add future password reset and email verification routes
    // if config.enable_password_reset {
    //     public_routes = public_routes
    //         .route("/users/forgot-password", post(forgot_password))
    //         .route("/users/reset-password", post(reset_password));
    // }

    (protected_routes, public_routes)
}
