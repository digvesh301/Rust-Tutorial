// Role domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: JsonValue,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Role {
    /// Create a new role instance
    pub fn new(name: String, description: Option<String>, permissions: JsonValue) -> Self {
        let now = Some(Utc::now());
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            permissions,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if role has a specific permission
    pub fn has_permission(&self, permission: &str) -> bool {
        if let Some(perms) = self.permissions.as_array() {
            // Check for wildcard permission
            if perms.iter().any(|p| p.as_str() == Some("*")) {
                return true;
            }
            // Check for specific permission
            perms.iter().any(|p| p.as_str() == Some(permission))
        } else {
            false
        }
    }

    /// Get all permissions as a vector of strings
    pub fn get_permissions(&self) -> Vec<String> {
        if let Some(perms) = self.permissions.as_array() {
            perms
                .iter()
                .filter_map(|p| p.as_str().map(|s| s.to_string()))
                .collect()
        } else {
            vec![]
        }
    }
}
