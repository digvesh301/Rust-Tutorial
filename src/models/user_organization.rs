// UserOrganization domain model for multi-tenant user-organization relationships

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserOrganization {
    pub id: Uuid,
    pub user_id: Uuid,
    pub org_id: Uuid,
    pub role_id: Uuid,
    pub status: String,
    pub joined_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserOrganizationWithDetails {
    // UserOrganization fields
    pub id: Uuid,
    pub user_id: Uuid,
    pub org_id: Uuid,
    pub role_id: Uuid,
    pub status: String,
    pub joined_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    
    // User details
    pub user_name: String,
    pub user_email: String,
    pub user_status: String,
    
    // Organization details
    pub org_name: String,
    pub org_country: Option<String>,
    pub org_timezone: Option<String>,
    
    // Role details
    pub role_name: String,
    pub role_description: Option<String>,
}

impl UserOrganization {
    /// Create a new user-organization relationship
    pub fn new(user_id: Uuid, org_id: Uuid, role_id: Uuid) -> Self {
        let now = Some(Utc::now());
        Self {
            id: Uuid::new_v4(),
            user_id,
            org_id,
            role_id,
            status: "active".to_string(),
            joined_at: now,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a new pending invitation
    pub fn new_invitation(user_id: Uuid, org_id: Uuid, role_id: Uuid) -> Self {
        let now = Some(Utc::now());
        Self {
            id: Uuid::new_v4(),
            user_id,
            org_id,
            role_id,
            status: "invited".to_string(),
            joined_at: None, // Will be set when invitation is accepted
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the relationship is active
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }

    /// Check if the relationship is pending
    pub fn is_pending(&self) -> bool {
        self.status == "pending"
    }

    /// Check if the relationship is an invitation
    pub fn is_invited(&self) -> bool {
        self.status == "invited"
    }

    /// Accept an invitation (change status to active and set joined_at)
    pub fn accept_invitation(&mut self) {
        self.status = "active".to_string();
        self.joined_at = Some(Utc::now());
        self.updated_at = Some(Utc::now());
    }

    /// Suspend the relationship
    pub fn suspend(&mut self) {
        self.status = "suspended".to_string();
        self.updated_at = Some(Utc::now());
    }

    /// Reactivate the relationship
    pub fn reactivate(&mut self) {
        self.status = "active".to_string();
        self.updated_at = Some(Utc::now());
    }

    /// Update the role
    pub fn update_role(&mut self, new_role_id: Uuid) {
        self.role_id = new_role_id;
        self.updated_at = Some(Utc::now());
    }
}
