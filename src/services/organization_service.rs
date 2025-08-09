// Organization Service - Business logic for organizations

use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{CreateOrganizationRequest, OrganizationResponse};
use crate::errors::AppError;
use crate::models::Organization;
use crate::repository::OrganizationRepository;
use crate::utils::format_timestamp;

pub struct OrganizationService;

impl OrganizationService {
    /// Create a new organization with validation
    pub async fn create_organization(
        pool: &PgPool,
        request: CreateOrganizationRequest,
    ) -> Result<OrganizationResponse, AppError> {
        // Validate required fields
        if request.name.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Organization name is required and cannot be empty".to_string(),
            ));
        }

        // Validate name length
        if request.name.trim().len() > 255 {
            return Err(AppError::ValidationError(
                "Organization name cannot exceed 255 characters".to_string(),
            ));
        }

        // Create organization using repository
        let organization = OrganizationRepository::create(
            pool,
            request.name.trim().to_string(),
            request.country,
            request.timezone,
        )
        .await?;

        // Convert to response DTO
        Ok(Self::to_response(organization))
    }

    /// Get organization by ID
    pub async fn get_organization_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<OrganizationResponse>, AppError> {
        let organization = OrganizationRepository::find_by_id(pool, id).await?;
        
        Ok(organization.map(Self::to_response))
    }

    /// Get all organizations
    pub async fn get_all_organizations(pool: &PgPool) -> Result<Vec<OrganizationResponse>, AppError> {
        let organizations = OrganizationRepository::find_all(pool).await?;
        
        Ok(organizations.into_iter().map(Self::to_response).collect())
    }

    /// Convert Organization model to response DTO
    fn to_response(organization: Organization) -> OrganizationResponse {
        OrganizationResponse {
            id: organization.id.to_string(),
            name: organization.name,
            country: organization.country,
            timezone: organization.timezone,
            created_at: format_timestamp(organization.created_at),
        }
    }
}
