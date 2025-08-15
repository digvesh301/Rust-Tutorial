use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

use crate::models::Contact;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateContactRequest {
    #[validate(length(min = 1, max = 100, message = "First name must be between 1 and 100 characters"))]
    pub first_name: String,

    #[validate(length(min = 1, max = 100, message = "Last name must be between 1 and 100 characters"))]
    pub last_name: String,

    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email must be less than 255 characters"))]
    pub email: String,

    #[validate(length(max = 20, message = "Phone must be less than 20 characters"))]
    pub phone: Option<String>,

    #[validate(length(max = 255, message = "Company must be less than 255 characters"))]
    pub company: Option<String>,

    #[validate(length(max = 100, message = "Job title must be less than 100 characters"))]
    pub job_title: Option<String>,

    pub address: Option<String>,

    #[validate(length(max = 100, message = "City must be less than 100 characters"))]
    pub city: Option<String>,

    #[validate(length(max = 100, message = "State must be less than 100 characters"))]
    pub state: Option<String>,

    #[validate(length(max = 20, message = "Postal code must be less than 20 characters"))]
    pub postal_code: Option<String>,

    #[validate(length(max = 100, message = "Country must be less than 100 characters"))]
    pub country: Option<String>,

    pub notes: Option<String>,

    #[validate(length(max = 100, message = "Lead source must be less than 100 characters"))]
    pub lead_source: Option<String>,

    #[validate(custom = "validate_lead_status")]
    pub lead_status: Option<String>,

    /// Custom field values - key is field_name, value is the field value
    pub custom_fields: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateContactRequest {
    #[validate(length(min = 1, max = 100, message = "First name must be between 1 and 100 characters"))]
    pub first_name: Option<String>,

    #[validate(length(min = 1, max = 100, message = "Last name must be between 1 and 100 characters"))]
    pub last_name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email must be less than 255 characters"))]
    pub email: Option<String>,

    #[validate(length(max = 20, message = "Phone must be less than 20 characters"))]
    pub phone: Option<String>,

    #[validate(length(max = 255, message = "Company must be less than 255 characters"))]
    pub company: Option<String>,

    #[validate(length(max = 100, message = "Job title must be less than 100 characters"))]
    pub job_title: Option<String>,

    pub address: Option<String>,

    #[validate(length(max = 100, message = "City must be less than 100 characters"))]
    pub city: Option<String>,

    #[validate(length(max = 100, message = "State must be less than 100 characters"))]
    pub state: Option<String>,

    #[validate(length(max = 20, message = "Postal code must be less than 20 characters"))]
    pub postal_code: Option<String>,

    #[validate(length(max = 100, message = "Country must be less than 100 characters"))]
    pub country: Option<String>,

    pub notes: Option<String>,

    #[validate(length(max = 100, message = "Lead source must be less than 100 characters"))]
    pub lead_source: Option<String>,

    pub lead_status: Option<String>,

    /// Custom field values - key is field_name, value is the field value
    pub custom_fields: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PatchContactRequest {
    #[validate(length(min = 1, max = 100, message = "First name must be between 1 and 100 characters"))]
    pub first_name: Option<String>,

    #[validate(length(min = 1, max = 100, message = "Last name must be between 1 and 100 characters"))]
    pub last_name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email must be less than 255 characters"))]
    pub email: Option<String>,

    #[validate(length(max = 20, message = "Phone must be less than 20 characters"))]
    pub phone: Option<String>,

    #[validate(length(max = 255, message = "Company must be less than 255 characters"))]
    pub company: Option<String>,

    #[validate(length(max = 100, message = "Job title must be less than 100 characters"))]
    pub job_title: Option<String>,

    pub address: Option<String>,

    #[validate(length(max = 100, message = "City must be less than 100 characters"))]
    pub city: Option<String>,

    #[validate(length(max = 100, message = "State must be less than 100 characters"))]
    pub state: Option<String>,

    #[validate(length(max = 20, message = "Postal code must be less than 20 characters"))]
    pub postal_code: Option<String>,

    #[validate(length(max = 100, message = "Country must be less than 100 characters"))]
    pub country: Option<String>,

    pub notes: Option<String>,

    #[validate(length(max = 100, message = "Lead source must be less than 100 characters"))]
    pub lead_source: Option<String>,

    pub lead_status: Option<String>,

    /// Custom field operations for PATCH
    /// - If present, will merge with existing custom fields
    /// - To remove a custom field, set its value to empty string
    pub custom_fields: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub notes: Option<String>,
    pub lead_source: Option<String>,
    pub lead_status: String,
    pub owner_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Custom field values
    pub custom_fields: Option<HashMap<String, String>>,
}

impl From<Contact> for ContactResponse {
    fn from(contact: Contact) -> Self {
        Self {
            id: contact.id,
            first_name: contact.first_name.clone(),
            last_name: contact.last_name.clone(),
            full_name: contact.full_name(),
            email: contact.email,
            phone: contact.phone,
            company: contact.company,
            job_title: contact.job_title,
            address: contact.address,
            city: contact.city,
            state: contact.state,
            postal_code: contact.postal_code,
            country: contact.country,
            notes: contact.notes,
            lead_source: contact.lead_source,
            lead_status: contact.lead_status,
            owner_id: contact.owner_id,
            is_active: contact.is_active,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
            custom_fields: None, // Will be populated separately
        }
    }
}

// Custom validator for lead status
fn validate_lead_status(lead_status: &str) -> Result<(), validator::ValidationError> {
    let valid_statuses = [
        "new",
        "contacted",
        "qualified",
        "proposal",
        "negotiation",
        "closed_won",
        "closed_lost"
    ];

    if valid_statuses.contains(&lead_status) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_lead_status"))
    }
}


