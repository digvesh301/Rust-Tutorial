use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::dto::contact_dto::{CreateContactRequest, ContactResponse};
use crate::errors::AppError;
use crate::models::Contact;
use crate::repository::contact_repository::ContactRepository;

pub struct ContactService;

impl ContactService {
    /// Create a new contact
    pub async fn create_contact(
        pool: &PgPool,
        request: CreateContactRequest,
        _created_by: Uuid, // User ID from JWT token
    ) -> Result<ContactResponse, AppError> {
        // Validate the request
        if let Err(validation_errors) = request.validate() {
            tracing::warn!("Contact validation failed: {:?}", validation_errors);
            return Err(AppError::ValidationError(
                "Invalid contact data".to_string()
            ));
        }

        // Check if email already exists
        if ContactRepository::email_exists(pool, &request.email).await? {
            tracing::warn!("Attempt to create contact with existing email: {}", request.email);
            return Err(AppError::ValidationError(
                "A contact with this email already exists".to_string()
            ));
        }

        // Create the contact model with the authenticated user as owner
        let mut contact = Contact::new(
            request.first_name,
            request.last_name,
            request.email,
            request.phone,
            request.company,
            request.job_title,
            Some(_created_by), // Set the authenticated user as the owner
        );

        // Set optional fields
        contact.address = request.address;
        contact.city = request.city;
        contact.state = request.state;
        contact.postal_code = request.postal_code;
        contact.country = request.country;
        contact.notes = request.notes;
        contact.lead_source = request.lead_source;
        
        // Set lead status if provided, otherwise use default "new"
        if let Some(status) = request.lead_status {
            contact.lead_status = status;
        }

        // Save to database
        let created_contact = ContactRepository::create(pool, &contact).await?;

        tracing::info!(
            "Contact created successfully: {} {} ({})", 
            created_contact.first_name, 
            created_contact.last_name,
            created_contact.email
        );

        Ok(ContactResponse::from(created_contact))
    }

    /// Get contact by ID
    pub async fn get_contact_by_id(
        pool: &PgPool,
        contact_id: Uuid,
        _user_id: Uuid, // For future authorization checks
    ) -> Result<ContactResponse, AppError> {
        let contact = ContactRepository::find_by_id(pool, contact_id).await?;

        match contact {
            Some(contact) => Ok(ContactResponse::from(contact)),
            None => {
                tracing::warn!("Contact not found with ID: {}", contact_id);
                Err(AppError::NotFound("Contact not found".to_string()))
            }
        }
    }



    /// Check if contact exists by email
    pub async fn contact_exists_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<bool, AppError> {
        ContactRepository::email_exists(pool, email).await
    }
}
