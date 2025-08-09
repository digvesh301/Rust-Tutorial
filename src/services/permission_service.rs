// Permission service for handling role-based access control

use crate::errors::AppError;
use crate::models::{Role, User, UserOrganization};
use serde_json::Value as JsonValue;
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

pub struct PermissionService;

impl PermissionService {
    /// Get all permissions for a user in a specific organization
    pub async fn get_user_permissions(
        pool: &PgPool,
        user_id: Uuid,
        org_id: Uuid,
    ) -> Result<HashSet<String>, AppError> {
        let query = r#"
            SELECT r.permissions
            FROM user_organizations uo
            JOIN roles r ON uo.role_id = r.id
            WHERE uo.user_id = $1 AND uo.org_id = $2 AND uo.status = 'active'
        "#;

        let rows = sqlx::query_as::<_, (JsonValue,)>(query)
            .bind(user_id)
            .bind(org_id)
            .fetch_all(pool)
            .await?;

        let mut permissions = HashSet::new();

        for (permission_json,) in rows {
            if let Some(perms) = permission_json.as_array() {
                for perm in perms {
                    if let Some(perm_str) = perm.as_str() {
                        permissions.insert(perm_str.to_string());
                    }
                }
            }
        }

        Ok(permissions)
    }

    /// Check if user has a specific permission in an organization
    pub async fn has_permission(
        pool: &PgPool,
        user_id: Uuid,
        org_id: Uuid,
        required_permission: &str,
    ) -> Result<bool, AppError> {
        let permissions = Self::get_user_permissions(pool, user_id, org_id).await?;

        // Check for wildcard permission
        if permissions.contains("*") {
            return Ok(true);
        }

        // Check for exact permission
        if permissions.contains(required_permission) {
            return Ok(true);
        }

        // Check for resource-level wildcard (e.g., "contacts:*")
        let parts: Vec<&str> = required_permission.split(':').collect();
        if parts.len() >= 2 {
            let resource_wildcard = format!("{}:*", parts[0]);
            if permissions.contains(&resource_wildcard) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Check if user has any of the specified permissions
    pub async fn has_any_permission(
        pool: &PgPool,
        user_id: Uuid,
        org_id: Uuid,
        required_permissions: &[&str],
    ) -> Result<bool, AppError> {
        for permission in required_permissions {
            if Self::has_permission(pool, user_id, org_id, permission).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Check if user has all of the specified permissions
    pub async fn has_all_permissions(
        pool: &PgPool,
        user_id: Uuid,
        org_id: Uuid,
        required_permissions: &[&str],
    ) -> Result<bool, AppError> {
        for permission in required_permissions {
            if !Self::has_permission(pool, user_id, org_id, permission).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Require permission (throws error if not authorized)
    pub async fn require_permission(
        pool: &PgPool,
        user_id: Uuid,
        org_id: Uuid,
        required_permission: &str,
    ) -> Result<(), AppError> {
        if !Self::has_permission(pool, user_id, org_id, required_permission).await? {
            return Err(AppError::Unauthorized(format!(
                "Permission '{}' required",
                required_permission
            )));
        }
        Ok(())
    }

    /// Check resource ownership (for "own" scoped permissions)
    pub async fn can_access_resource(
        pool: &PgPool,
        user_id: Uuid,
        org_id: Uuid,
        resource_owner_id: Option<Uuid>,
        base_permission: &str,
    ) -> Result<bool, AppError> {
        // Check if user has full permission
        if Self::has_permission(pool, user_id, org_id, base_permission).await? {
            return Ok(true);
        }

        // Check if user has "own" permission and owns the resource
        let own_permission = format!("{}:own", base_permission);
        if Self::has_permission(pool, user_id, org_id, &own_permission).await? {
            if let Some(owner_id) = resource_owner_id {
                return Ok(owner_id == user_id);
            }
        }

        Ok(false)
    }

    /// Get user's roles in an organization
    pub async fn get_user_roles(
        pool: &PgPool,
        user_id: Uuid,
        org_id: Uuid,
    ) -> Result<Vec<Role>, AppError> {
        let query = r#"
            SELECT r.id, r.name, r.description, r.permissions, r.created_at, r.updated_at
            FROM user_organizations uo
            JOIN roles r ON uo.role_id = r.id
            WHERE uo.user_id = $1 AND uo.org_id = $2 AND uo.status = 'active'
        "#;

        let roles = sqlx::query_as::<_, Role>(query)
            .bind(user_id)
            .bind(org_id)
            .fetch_all(pool)
            .await?;

        Ok(roles)
    }
}

/// Permission checking macros for easier usage
#[macro_export]
macro_rules! require_permission {
    ($pool:expr, $user_id:expr, $org_id:expr, $permission:expr) => {
        crate::services::permission_service::PermissionService::require_permission(
            $pool, $user_id, $org_id, $permission,
        )
        .await?
    };
}

#[macro_export]
macro_rules! has_permission {
    ($pool:expr, $user_id:expr, $org_id:expr, $permission:expr) => {
        crate::services::permission_service::PermissionService::has_permission(
            $pool, $user_id, $org_id, $permission,
        )
        .await?
    };
}
