// Routes module - Organize API routes by feature
pub mod contact_routes;
pub mod organization_routes;
pub mod user_routes;
pub mod user_organization_routes;

pub use contact_routes::{contact_routes, contact_routes_with_permissions};
pub use organization_routes::*;
pub use user_routes::*;
pub use user_organization_routes::*;
