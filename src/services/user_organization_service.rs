// UserOrganization Service - Business logic for user-organization relationships

use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{
    CreateUserOrganizationRequest, InviteUserToOrganizationRequest, 
    UpdateUserOrganizationRequest, UserOrganizationDetailResponse,
    UserOrganizationListResponse, UserOrganizationResponse,
    UserInfo, OrganizationInfo, RoleInfo
};
use crate::errors::AppError;
use crate::models::{UserOrganization, UserOrganizationWithDetails};
use crate::repository::{
    RoleRepository, UserOrganizationRepository, UserRepository, OrganizationRepository
};
use crate::utils::format_timestamp;

pub struct UserOrganizationService;

impl UserOrganizationService {
    /// Add user to organization
    pub async fn add_user_to_organization(
        pool: &PgPool,
        request: CreateUserOrganizationRequest,
    ) -> Result<UserOrganizationDetailResponse, AppError> {
        // Validate user exists
        let user = UserRepository::find_by_id(pool, request.user_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", request.user_id)));
        }

        // Validate organization exists
        let org = OrganizationRepository::find_by_id(pool, request.org_id).await?;
        if org.is_none() {
            return Err(AppError::NotFound(format!("Organization with id {} not found", request.org_id)));
        }

        // Get role by name
        let role = RoleRepository::find_by_name(pool, &request.role_name).await?;
        let role = match role {
            Some(role) => role,
            None => return Err(AppError::NotFound(format!("Role '{}' not found", request.role_name))),
        };

        // Check if relationship already exists
        let existing = UserOrganizationRepository::find_by_user_and_org(
            pool, 
            request.user_id, 
            request.org_id
        ).await?;

        if existing.is_some() {
            return Err(AppError::ValidationError(
                "User is already associated with this organization".to_string()
            ));
        }

        // Create the relationship
        let _user_org = UserOrganizationRepository::create(
            pool,
            request.user_id,
            request.org_id,
            role.id,
            Some("active".to_string()),
        ).await?;

        // Get detailed information for response
        let detailed = UserOrganizationRepository::find_all_with_details(
            pool,
            Some(request.user_id),
            Some(request.org_id),
            None,
            1,
            0,
        ).await?;

        if let Some(detail) = detailed.first() {
            Ok(Self::to_detail_response(detail.clone()))
        } else {
            Err(AppError::InternalServerError("Failed to retrieve created relationship".to_string()))
        }
    }

    /// Invite user to organization
    pub async fn invite_user_to_organization(
        pool: &PgPool,
        request: InviteUserToOrganizationRequest,
    ) -> Result<UserOrganizationDetailResponse, AppError> {
        // Find user by email
        let user = UserRepository::find_by_email(pool, &request.email).await?;
        let user = match user {
            Some(user) => user,
            None => return Err(AppError::NotFound(format!("User with email '{}' not found", request.email))),
        };

        // Validate organization exists
        let org = OrganizationRepository::find_by_id(pool, request.org_id).await?;
        if org.is_none() {
            return Err(AppError::NotFound(format!("Organization with id {} not found", request.org_id)));
        }

        // Get role by name
        let role = RoleRepository::find_by_name(pool, &request.role_name).await?;
        let role = match role {
            Some(role) => role,
            None => return Err(AppError::NotFound(format!("Role '{}' not found", request.role_name))),
        };

        // Check if relationship already exists
        let existing = UserOrganizationRepository::find_by_user_and_org(
            pool, 
            user.id, 
            request.org_id
        ).await?;

        if existing.is_some() {
            return Err(AppError::ValidationError(
                "User is already associated with this organization".to_string()
            ));
        }

        // Create invitation
        let _user_org = UserOrganizationRepository::create(
            pool,
            user.id,
            request.org_id,
            role.id,
            Some("invited".to_string()),
        ).await?;

        // Get detailed information for response
        let detailed = UserOrganizationRepository::find_all_with_details(
            pool,
            Some(user.id),
            Some(request.org_id),
            None,
            1,
            0,
        ).await?;

        if let Some(detail) = detailed.first() {
            Ok(Self::to_detail_response(detail.clone()))
        } else {
            Err(AppError::InternalServerError("Failed to retrieve created invitation".to_string()))
        }
    }

    /// Get user organizations
    pub async fn get_user_organizations(
        pool: &PgPool,
        user_id: Uuid,
        status: Option<String>,
    ) -> Result<Vec<UserOrganizationDetailResponse>, AppError> {
        let relationships = UserOrganizationRepository::find_organizations_for_user(
            pool, 
            user_id, 
            status
        ).await?;

        let responses = relationships
            .into_iter()
            .map(Self::to_detail_response)
            .collect();

        Ok(responses)
    }

    /// Get organization users
    pub async fn get_organization_users(
        pool: &PgPool,
        org_id: Uuid,
        status: Option<String>,
    ) -> Result<Vec<UserOrganizationDetailResponse>, AppError> {
        let relationships = UserOrganizationRepository::find_users_for_organization(
            pool, 
            org_id, 
            status
        ).await?;

        let responses = relationships
            .into_iter()
            .map(Self::to_detail_response)
            .collect();

        Ok(responses)
    }

    /// Update user organization relationship
    pub async fn update_user_organization(
        pool: &PgPool,
        id: Uuid,
        request: UpdateUserOrganizationRequest,
    ) -> Result<UserOrganizationDetailResponse, AppError> {
        // Check if relationship exists
        let existing = UserOrganizationRepository::find_by_id(pool, id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound(format!("User organization relationship with id {} not found", id)));
        }

        let mut role_id = None;
        
        // Resolve role name to role ID if provided
        if let Some(role_name) = &request.role_name {
            let role = RoleRepository::find_by_name(pool, role_name).await?;
            role_id = Some(match role {
                Some(role) => role.id,
                None => return Err(AppError::NotFound(format!("Role '{}' not found", role_name))),
            });
        }

        // Update the relationship
        let _updated = UserOrganizationRepository::update(
            pool,
            id,
            role_id,
            request.status,
        ).await?;

        // Get detailed information for response
        let existing = existing.unwrap();
        let detailed = UserOrganizationRepository::find_all_with_details(
            pool,
            Some(existing.user_id),
            Some(existing.org_id),
            None,
            1,
            0,
        ).await?;

        if let Some(detail) = detailed.first() {
            Ok(Self::to_detail_response(detail.clone()))
        } else {
            Err(AppError::InternalServerError("Failed to retrieve updated relationship".to_string()))
        }
    }

    /// Remove user from organization
    pub async fn remove_user_from_organization(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), AppError> {
        // Check if relationship exists
        let existing = UserOrganizationRepository::find_by_id(pool, id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound(format!("User organization relationship with id {} not found", id)));
        }

        UserOrganizationRepository::delete(pool, id).await?;
        Ok(())
    }

    /// Convert UserOrganizationWithDetails to response DTO
    fn to_detail_response(detail: UserOrganizationWithDetails) -> UserOrganizationDetailResponse {
        UserOrganizationDetailResponse {
            id: detail.id,
            user_id: detail.user_id,
            org_id: detail.org_id,
            role_id: detail.role_id,
            status: detail.status,
            joined_at: detail.joined_at.map(|dt| format_timestamp(Some(dt))),
            created_at: format_timestamp(detail.created_at),
            updated_at: format_timestamp(detail.updated_at),
            user: UserInfo {
                id: detail.user_id,
                name: detail.user_name,
                email: detail.user_email,
                status: detail.user_status,
            },
            organization: OrganizationInfo {
                id: detail.org_id,
                name: detail.org_name,
                country: detail.org_country,
                timezone: detail.org_timezone,
            },
            role: RoleInfo {
                id: detail.role_id,
                name: detail.role_name,
                description: detail.role_description,
                permissions: vec![], // TODO: Parse permissions from role
            },
        }
    }

    /// Convert UserOrganization to response DTO
    fn to_response(user_org: UserOrganization) -> UserOrganizationResponse {
        UserOrganizationResponse {
            id: user_org.id,
            user_id: user_org.user_id,
            org_id: user_org.org_id,
            role_id: user_org.role_id,
            status: user_org.status,
            joined_at: user_org.joined_at.map(|dt| format_timestamp(Some(dt))),
            created_at: format_timestamp(user_org.created_at),
            updated_at: format_timestamp(user_org.updated_at),
        }
    }
}
