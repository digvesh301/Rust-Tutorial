// UserOrganization Repository - Database operations for user-organization relationships

use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{UserOrganization, UserOrganizationWithDetails};

pub struct UserOrganizationRepository;

impl UserOrganizationRepository {
    /// Create user-organization relationship
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        org_id: Uuid,
        role_id: Uuid,
        status: Option<String>,
    ) -> Result<UserOrganization, AppError> {
        let status = status.unwrap_or_else(|| "active".to_string());
        
        let result = sqlx::query_as::<_, UserOrganization>(
            r#"
            INSERT INTO user_organizations (user_id, org_id, role_id, status)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, org_id, role_id, status, joined_at, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(org_id)
        .bind(role_id)
        .bind(status)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// Find user-organization relationship by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserOrganization>, AppError> {
        let result = sqlx::query_as::<_, UserOrganization>(
            r#"
            SELECT id, user_id, org_id, role_id, status, joined_at, created_at, updated_at
            FROM user_organizations
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Find user-organization relationship by user_id and org_id
    pub async fn find_by_user_and_org(
        pool: &PgPool,
        user_id: Uuid,
        org_id: Uuid,
    ) -> Result<Option<UserOrganization>, AppError> {
        let result = sqlx::query_as::<_, UserOrganization>(
            r#"
            SELECT id, user_id, org_id, role_id, status, joined_at, created_at, updated_at
            FROM user_organizations
            WHERE user_id = $1 AND org_id = $2
            "#,
        )
        .bind(user_id)
        .bind(org_id)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Get all user-organization relationships with details
    pub async fn find_all_with_details(
        pool: &PgPool,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserOrganizationWithDetails>, AppError> {
        let mut query = r#"
            SELECT 
                uo.id, uo.user_id, uo.org_id, uo.role_id, uo.status, 
                uo.joined_at, uo.created_at, uo.updated_at,
                u.name as user_name, u.email as user_email, u.status as user_status,
                o.name as org_name, o.country as org_country, o.timezone as org_timezone,
                r.name as role_name, r.description as role_description
            FROM user_organizations uo
            JOIN users u ON uo.user_id = u.id
            JOIN organization o ON uo.org_id = o.id
            JOIN roles r ON uo.role_id = r.id
            WHERE 1=1
        "#.to_string();

        let mut param_count = 1;
        let mut conditions = Vec::new();

        if user_id.is_some() {
            conditions.push(format!("AND uo.user_id = ${}", param_count));
            param_count += 1;
        }

        if org_id.is_some() {
            conditions.push(format!("AND uo.org_id = ${}", param_count));
            param_count += 1;
        }

        if status.is_some() {
            conditions.push(format!("AND uo.status = ${}", param_count));
            param_count += 1;
        }

        for condition in conditions {
            query.push_str(&condition);
        }

        query.push_str(&format!(
            " ORDER BY uo.created_at DESC LIMIT ${} OFFSET ${}",
            param_count, param_count + 1
        ));

        let mut db_query = sqlx::query_as::<_, UserOrganizationWithDetails>(&query);

        if let Some(uid) = user_id {
            db_query = db_query.bind(uid);
        }
        if let Some(oid) = org_id {
            db_query = db_query.bind(oid);
        }
        if let Some(stat) = status {
            db_query = db_query.bind(stat);
        }

        let results = db_query
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok(results)
    }

    /// Get organizations for a user
    pub async fn find_organizations_for_user(
        pool: &PgPool,
        user_id: Uuid,
        status: Option<String>,
    ) -> Result<Vec<UserOrganizationWithDetails>, AppError> {
        let mut query = r#"
            SELECT 
                uo.id, uo.user_id, uo.org_id, uo.role_id, uo.status, 
                uo.joined_at, uo.created_at, uo.updated_at,
                u.name as user_name, u.email as user_email, u.status as user_status,
                o.name as org_name, o.country as org_country, o.timezone as org_timezone,
                r.name as role_name, r.description as role_description
            FROM user_organizations uo
            JOIN users u ON uo.user_id = u.id
            JOIN organization o ON uo.org_id = o.id
            JOIN roles r ON uo.role_id = r.id
            WHERE uo.user_id = $1
        "#.to_string();

        if let Some(_) = status {
            query.push_str(" AND uo.status = $2");
        }

        query.push_str(" ORDER BY uo.joined_at DESC");

        let mut db_query = sqlx::query_as::<_, UserOrganizationWithDetails>(&query).bind(user_id);

        if let Some(stat) = status {
            db_query = db_query.bind(stat);
        }

        let results = db_query.fetch_all(pool).await?;

        Ok(results)
    }

    /// Get users for an organization
    pub async fn find_users_for_organization(
        pool: &PgPool,
        org_id: Uuid,
        status: Option<String>,
    ) -> Result<Vec<UserOrganizationWithDetails>, AppError> {
        let mut query = r#"
            SELECT 
                uo.id, uo.user_id, uo.org_id, uo.role_id, uo.status, 
                uo.joined_at, uo.created_at, uo.updated_at,
                u.name as user_name, u.email as user_email, u.status as user_status,
                o.name as org_name, o.country as org_country, o.timezone as org_timezone,
                r.name as role_name, r.description as role_description
            FROM user_organizations uo
            JOIN users u ON uo.user_id = u.id
            JOIN organization o ON uo.org_id = o.id
            JOIN roles r ON uo.role_id = r.id
            WHERE uo.org_id = $1
        "#.to_string();

        if let Some(_) = status {
            query.push_str(" AND uo.status = $2");
        }

        query.push_str(" ORDER BY uo.joined_at DESC");

        let mut db_query = sqlx::query_as::<_, UserOrganizationWithDetails>(&query).bind(org_id);

        if let Some(stat) = status {
            db_query = db_query.bind(stat);
        }

        let results = db_query.fetch_all(pool).await?;

        Ok(results)
    }

    /// Update user-organization relationship
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        role_id: Option<Uuid>,
        status: Option<String>,
    ) -> Result<UserOrganization, AppError> {
        // Simple approach - update based on what's provided
        match (role_id, status) {
            (Some(rid), Some(stat)) => {
                sqlx::query_as::<_, UserOrganization>(
                    r#"
                    UPDATE user_organizations
                    SET role_id = $1, status = $2, updated_at = NOW()
                    WHERE id = $3
                    RETURNING id, user_id, org_id, role_id, status, joined_at, created_at, updated_at
                    "#,
                )
                .bind(rid)
                .bind(stat)
                .bind(id)
                .fetch_one(pool)
                .await
            }
            (Some(rid), None) => {
                sqlx::query_as::<_, UserOrganization>(
                    r#"
                    UPDATE user_organizations
                    SET role_id = $1, updated_at = NOW()
                    WHERE id = $2
                    RETURNING id, user_id, org_id, role_id, status, joined_at, created_at, updated_at
                    "#,
                )
                .bind(rid)
                .bind(id)
                .fetch_one(pool)
                .await
            }
            (None, Some(stat)) => {
                sqlx::query_as::<_, UserOrganization>(
                    r#"
                    UPDATE user_organizations
                    SET status = $1, updated_at = NOW()
                    WHERE id = $2
                    RETURNING id, user_id, org_id, role_id, status, joined_at, created_at, updated_at
                    "#,
                )
                .bind(stat)
                .bind(id)
                .fetch_one(pool)
                .await
            }
            (None, None) => {
                sqlx::query_as::<_, UserOrganization>(
                    r#"
                    UPDATE user_organizations
                    SET updated_at = NOW()
                    WHERE id = $1
                    RETURNING id, user_id, org_id, role_id, status, joined_at, created_at, updated_at
                    "#,
                )
                .bind(id)
                .fetch_one(pool)
                .await
            }
        }
        .map_err(AppError::from)
    }

    /// Delete user-organization relationship
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM user_organizations WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Count total relationships with filters
    pub async fn count_with_filters(
        pool: &PgPool,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
        status: Option<String>,
    ) -> Result<i64, AppError> {
        let mut query = "SELECT COUNT(*) FROM user_organizations WHERE 1=1".to_string();
        let mut param_count = 1;

        if user_id.is_some() {
            query.push_str(&format!(" AND user_id = ${}", param_count));
            param_count += 1;
        }

        if org_id.is_some() {
            query.push_str(&format!(" AND org_id = ${}", param_count));
            param_count += 1;
        }

        if status.is_some() {
            query.push_str(&format!(" AND status = ${}", param_count));
        }

        let mut db_query = sqlx::query(&query);

        if let Some(uid) = user_id {
            db_query = db_query.bind(uid);
        }
        if let Some(oid) = org_id {
            db_query = db_query.bind(oid);
        }
        if let Some(stat) = status {
            db_query = db_query.bind(stat);
        }

        let result = db_query.fetch_one(pool).await?;
        let count: i64 = result.get(0);

        Ok(count)
    }
}
