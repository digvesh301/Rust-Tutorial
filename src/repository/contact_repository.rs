use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::Contact;

pub struct ContactRepository;

impl ContactRepository {
    /// Create a new contact
    pub async fn create(pool: &PgPool, contact: &Contact) -> Result<Contact, AppError> {
        let query = r#"
            INSERT INTO contacts (
                id, first_name, last_name, email, phone, company, job_title,
                address, city, state, postal_code, country, notes,
                lead_source, lead_status, owner_id, is_active, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            RETURNING *
        "#;

        let result = sqlx::query_as::<_, Contact>(query)
            .bind(&contact.id)
            .bind(&contact.first_name)
            .bind(&contact.last_name)
            .bind(&contact.email)
            .bind(&contact.phone)
            .bind(&contact.company)
            .bind(&contact.job_title)
            .bind(&contact.address)
            .bind(&contact.city)
            .bind(&contact.state)
            .bind(&contact.postal_code)
            .bind(&contact.country)
            .bind(&contact.notes)
            .bind(&contact.lead_source)
            .bind(&contact.lead_status)
            .bind(&contact.owner_id)
            .bind(&contact.is_active)
            .bind(&contact.created_at)
            .bind(&contact.updated_at)
            .fetch_one(pool)
            .await;

        match result {
            Ok(contact) => {
                tracing::info!("Contact created successfully with ID: {}", contact.id);
                Ok(contact)
            }
            Err(sqlx::Error::Database(db_err)) => {
                if db_err.constraint().is_some() {
                    tracing::warn!("Contact creation failed - email already exists: {}", contact.email);
                    Err(AppError::ValidationError("Email already exists".to_string()))
                } else {
                    tracing::error!("Database error creating contact: {}", db_err);
                    Err(AppError::DatabaseError(sqlx::Error::Database(db_err)))
                }
            }
            Err(e) => {
                tracing::error!("Error creating contact: {}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Find contact by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Contact>, AppError> {
        let query = "SELECT * FROM contacts WHERE id = $1 AND is_active = true";
        
        let result = sqlx::query_as::<_, Contact>(query)
            .bind(id)
            .fetch_optional(pool)
            .await;

        match result {
            Ok(contact) => Ok(contact),
            Err(e) => {
                tracing::error!("Error finding contact by ID {}: {}", id, e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Find contact by email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Contact>, AppError> {
        let query = "SELECT * FROM contacts WHERE email = $1 AND is_active = true";
        
        let result = sqlx::query_as::<_, Contact>(query)
            .bind(email)
            .fetch_optional(pool)
            .await;

        match result {
            Ok(contact) => Ok(contact),
            Err(e) => {
                tracing::error!("Error finding contact by email {}: {}", email, e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Check if email exists
    pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, AppError> {
        let query = "SELECT EXISTS(SELECT 1 FROM contacts WHERE email = $1 AND is_active = true)";

        let result = sqlx::query_scalar::<_, bool>(query)
            .bind(email)
            .fetch_one(pool)
            .await;

        match result {
            Ok(exists) => Ok(exists),
            Err(e) => {
                tracing::error!("Error checking if email exists {}: {}", email, e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Update an existing contact
    pub async fn update(pool: &PgPool, contact: &Contact) -> Result<Contact, AppError> {
        let query = r#"
            UPDATE contacts SET
                first_name = $2,
                last_name = $3,
                email = $4,
                phone = $5,
                company = $6,
                job_title = $7,
                address = $8,
                city = $9,
                state = $10,
                postal_code = $11,
                country = $12,
                notes = $13,
                lead_source = $14,
                lead_status = $15,
                updated_at = $16
            WHERE id = $1 AND is_active = true
            RETURNING *
        "#;

        let result = sqlx::query_as::<_, Contact>(query)
            .bind(&contact.id)
            .bind(&contact.first_name)
            .bind(&contact.last_name)
            .bind(&contact.email)
            .bind(&contact.phone)
            .bind(&contact.company)
            .bind(&contact.job_title)
            .bind(&contact.address)
            .bind(&contact.city)
            .bind(&contact.state)
            .bind(&contact.postal_code)
            .bind(&contact.country)
            .bind(&contact.notes)
            .bind(&contact.lead_source)
            .bind(&contact.lead_status)
            .bind(&contact.updated_at)
            .fetch_one(pool)
            .await;

        match result {
            Ok(contact) => {
                tracing::info!("Contact updated successfully with ID: {}", contact.id);
                Ok(contact)
            }
            Err(sqlx::Error::RowNotFound) => {
                tracing::warn!("No contact found to update with ID: {}", contact.id);
                Err(AppError::NotFound("Contact not found".to_string()))
            }
            Err(e) => {
                tracing::error!("Error updating contact {}: {}", contact.id, e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    /// Soft delete contact by setting is_active to false
    pub async fn soft_delete(pool: &PgPool, contact_id: Uuid) -> Result<(), AppError> {
        let query = r#"
            UPDATE contacts
            SET is_active = false, updated_at = NOW()
            WHERE id = $1 AND is_active = true
        "#;

        let result = sqlx::query(query)
            .bind(contact_id)
            .execute(pool)
            .await;

        match result {
            Ok(query_result) => {
                if query_result.rows_affected() == 0 {
                    tracing::warn!("No contact found to delete with ID: {}", contact_id);
                    Err(AppError::NotFound("Contact not found".to_string()))
                } else {
                    tracing::info!("Contact soft deleted successfully with ID: {}", contact_id);
                    Ok(())
                }
            }
            Err(e) => {
                tracing::error!("Error soft deleting contact {}: {}", contact_id, e);
                Err(AppError::DatabaseError(e))
            }
        }
    }
}
