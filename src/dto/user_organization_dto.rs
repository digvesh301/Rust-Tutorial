// User Organization Data Transfer Objects

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserOrganizationRequest {
    pub user_id: Uuid,
    pub org_id: Uuid,
    pub role_name: String, // We'll resolve this to role_id in the service
}

#[derive(Debug, Deserialize)]
pub struct InviteUserToOrganizationRequest {
    pub email: String, // Email of user to invite
    pub org_id: Uuid,
    pub role_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserOrganizationRequest {
    pub role_name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserOrganizationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub org_id: Uuid,
    pub role_id: Uuid,
    pub status: String,
    pub joined_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct UserOrganizationDetailResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub org_id: Uuid,
    pub role_id: Uuid,
    pub status: String,
    pub joined_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    
    // User details
    pub user: UserInfo,
    
    // Organization details
    pub organization: OrganizationInfo,
    
    // Role details
    pub role: RoleInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct OrganizationInfo {
    pub id: Uuid,
    pub name: String,
    pub country: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RoleInfo {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserOrganizationQueryParams {
    pub user_id: Option<Uuid>,
    pub org_id: Option<Uuid>,
    pub role_name: Option<String>,
    pub status: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct UserOrganizationListResponse {
    pub data: Vec<UserOrganizationDetailResponse>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}
