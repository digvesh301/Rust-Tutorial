// User domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)] // Don't serialize password in responses
    pub password: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    /// Create a new user instance
    pub fn new(name: String, email: String, password: String) -> Self {
        let now = Some(Utc::now());
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            password,
            status: "active".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Convert User to UserResponse (without password)
    pub fn to_response(&self) -> UserResponse {
        UserResponse {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            status: self.status.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    /// Check if user is active
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }

    /// Update user status
    pub fn set_status(&mut self, status: String) {
        self.status = status;
        self.updated_at = Some(Utc::now());
    }
}
