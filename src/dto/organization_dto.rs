// Organization Data Transfer Objects

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub country: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Serialize)]
pub struct OrganizationResponse {
    pub id: String,
    pub name: String,
    pub country: Option<String>,
    pub timezone: Option<String>,
    pub created_at: String,
}
