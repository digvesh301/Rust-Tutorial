use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use chrono::Utc;

use crate::dto::contact_dto::{CreateContactRequest, ContactResponse, UpdateContactRequest, PatchContactRequest};
use crate::errors::AppError;
use crate::models::{Contact, ContactCustomValue};
use crate::repository::{ContactRepository, ContactCustomValueRepository};
use std::collections::HashMap;

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

        // Handle custom fields if provided
        if let Some(custom_fields) = request.custom_fields {
            Self::save_custom_fields(pool, created_contact.id, custom_fields).await?;
        }

        tracing::info!(
            "Contact created successfully: {} {} ({})",
            created_contact.first_name,
            created_contact.last_name,
            created_contact.email
        );

        // Get contact with custom fields for response
        let mut response = ContactResponse::from(created_contact.clone());
        response.custom_fields = Self::get_custom_fields_for_contact(pool, created_contact.id).await?;

        Ok(response)
    }

    /// Get contact by ID
    pub async fn get_contact_by_id(
        pool: &PgPool,
        contact_id: Uuid,
        _user_id: Uuid, // For future authorization checks
    ) -> Result<ContactResponse, AppError> {
        let contact = ContactRepository::find_by_id(pool, contact_id).await?;

        match contact {
            Some(contact) => {
                let mut response = ContactResponse::from(contact);
                response.custom_fields = Self::get_custom_fields_for_contact(pool, contact_id).await?;
                Ok(response)
            }
            None => {
                tracing::warn!("Contact not found with ID: {}", contact_id);
                Err(AppError::NotFound("Contact not found".to_string()))
            }
        }
    }

    /// Update an existing contact
    pub async fn update_contact(
        pool: &PgPool,
        contact_id: Uuid,
        request: UpdateContactRequest,
        _user_id: Uuid, // For future authorization checks
    ) -> Result<ContactResponse, AppError> {
        // Validate the request
        request.validate().map_err(|e| {
            tracing::warn!("Contact update validation failed: {:?}", e);
            AppError::ValidationError(e.to_string())
        })?;

        // Get the existing contact
        let existing_contact = ContactRepository::find_by_id(pool, contact_id).await?;

        let mut contact = match existing_contact {
            Some(contact) => {
                if !contact.is_active {
                    tracing::warn!("Attempt to update inactive contact: {}", contact_id);
                    return Err(AppError::NotFound("Contact not found".to_string()));
                }
                contact
            }
            None => {
                tracing::warn!("Contact not found with ID: {}", contact_id);
                return Err(AppError::NotFound("Contact not found".to_string()));
            }
        };

        // Update fields if provided
        if let Some(first_name) = request.first_name {
            contact.first_name = first_name;
        }
        if let Some(last_name) = request.last_name {
            contact.last_name = last_name;
        }
        if let Some(email) = request.email {
            contact.email = email;
        }
        if let Some(phone) = request.phone {
            contact.phone = Some(phone);
        }
        if let Some(company) = request.company {
            contact.company = Some(company);
        }
        if let Some(job_title) = request.job_title {
            contact.job_title = Some(job_title);
        }
        if let Some(address) = request.address {
            contact.address = Some(address);
        }
        if let Some(city) = request.city {
            contact.city = Some(city);
        }
        if let Some(state) = request.state {
            contact.state = Some(state);
        }
        if let Some(postal_code) = request.postal_code {
            contact.postal_code = Some(postal_code);
        }
        if let Some(country) = request.country {
            contact.country = Some(country);
        }
        if let Some(notes) = request.notes {
            contact.notes = Some(notes);
        }
        if let Some(lead_source) = request.lead_source {
            contact.lead_source = Some(lead_source);
        }
        if let Some(lead_status) = request.lead_status {
            // Validate lead status
            let valid_statuses = [
                "new", "contacted", "qualified", "proposal",
                "negotiation", "closed_won", "closed_lost"
            ];
            if !valid_statuses.contains(&lead_status.as_str()) {
                return Err(AppError::ValidationError(
                    "Invalid lead status. Must be one of: new, contacted, qualified, proposal, negotiation, closed_won, closed_lost".to_string()
                ));
            }
            contact.lead_status = lead_status;
        }

        // Update the timestamp
        contact.updated_at = Utc::now();

        // Save to database
        let updated_contact = ContactRepository::update(pool, &contact).await?;

        // Handle custom fields if provided
        if let Some(custom_fields) = request.custom_fields {
            Self::save_custom_fields(pool, updated_contact.id, custom_fields).await?;
        }

        tracing::info!(
            "Contact updated successfully: {} {} ({})",
            updated_contact.first_name,
            updated_contact.last_name,
            updated_contact.email
        );

        // Get contact with custom fields for response
        let mut response = ContactResponse::from(updated_contact.clone());
        response.custom_fields = Self::get_custom_fields_for_contact(pool, updated_contact.id).await?;

        Ok(response)
    }

    /// Patch an existing contact (partial update with merge semantics)
    pub async fn patch_contact(
        pool: &PgPool,
        contact_id: Uuid,
        request: PatchContactRequest,
        _user_id: Uuid, // For future authorization checks
    ) -> Result<ContactResponse, AppError> {
        // Validate the request
        request.validate().map_err(|e| {
            tracing::warn!("Contact patch validation failed: {:?}", e);
            AppError::ValidationError(e.to_string())
        })?;

        // Get the existing contact
        let existing_contact = ContactRepository::find_by_id(pool, contact_id).await?;

        let mut contact = match existing_contact {
            Some(contact) => {
                if !contact.is_active {
                    tracing::warn!("Attempt to patch inactive contact: {}", contact_id);
                    return Err(AppError::NotFound("Contact not found".to_string()));
                }
                contact
            }
            None => {
                tracing::warn!("Contact not found with ID: {}", contact_id);
                return Err(AppError::NotFound("Contact not found".to_string()));
            }
        };

        // Apply patches if provided
        if let Some(first_name) = request.first_name {
            contact.first_name = first_name;
        }
        if let Some(last_name) = request.last_name {
            contact.last_name = last_name;
        }
        if let Some(email) = request.email {
            contact.email = email;
        }
        if let Some(phone) = request.phone {
            contact.phone = if phone.trim().is_empty() { None } else { Some(phone) };
        }
        if let Some(company) = request.company {
            contact.company = if company.trim().is_empty() { None } else { Some(company) };
        }
        if let Some(job_title) = request.job_title {
            contact.job_title = if job_title.trim().is_empty() { None } else { Some(job_title) };
        }
        if let Some(address) = request.address {
            contact.address = if address.trim().is_empty() { None } else { Some(address) };
        }
        if let Some(city) = request.city {
            contact.city = if city.trim().is_empty() { None } else { Some(city) };
        }
        if let Some(state) = request.state {
            contact.state = if state.trim().is_empty() { None } else { Some(state) };
        }
        if let Some(postal_code) = request.postal_code {
            contact.postal_code = if postal_code.trim().is_empty() { None } else { Some(postal_code) };
        }
        if let Some(country) = request.country {
            contact.country = if country.trim().is_empty() { None } else { Some(country) };
        }
        if let Some(notes) = request.notes {
            contact.notes = if notes.trim().is_empty() { None } else { Some(notes) };
        }
        if let Some(lead_source) = request.lead_source {
            contact.lead_source = if lead_source.trim().is_empty() { None } else { Some(lead_source) };
        }
        if let Some(lead_status) = request.lead_status {
            // Validate lead status
            let valid_statuses = [
                "new", "contacted", "qualified", "proposal",
                "negotiation", "closed_won", "closed_lost"
            ];
            if !valid_statuses.contains(&lead_status.as_str()) {
                return Err(AppError::ValidationError(
                    "Invalid lead status. Must be one of: new, contacted, qualified, proposal, negotiation, closed_won, closed_lost".to_string()
                ));
            }
            contact.lead_status = lead_status;
        }

        // Update the timestamp
        contact.updated_at = Utc::now();

        // Save to database
        let updated_contact = ContactRepository::update(pool, &contact).await?;

        // Handle custom fields if provided (merge semantics for PATCH)
        if let Some(custom_fields) = request.custom_fields {
            Self::patch_custom_fields(pool, updated_contact.id, custom_fields).await?;
        }

        tracing::info!(
            "Contact patched successfully: {} {} ({})",
            updated_contact.first_name,
            updated_contact.last_name,
            updated_contact.email
        );

        // Get contact with custom fields for response
        let mut response = ContactResponse::from(updated_contact.clone());
        response.custom_fields = Self::get_custom_fields_for_contact(pool, updated_contact.id).await?;

        Ok(response)
    }

    /// Delete contact by ID (soft delete by setting is_active to false)
    pub async fn delete_contact(
        pool: &PgPool,
        contact_id: Uuid,
        _user_id: Uuid, // For future authorization checks
    ) -> Result<(), AppError> {
        // Check if contact exists and is active
        let contact = ContactRepository::find_by_id(pool, contact_id).await?;

        match contact {
            Some(contact) => {
                if !contact.is_active {
                    tracing::warn!("Attempt to delete already inactive contact: {}", contact_id);
                    return Err(AppError::NotFound("Contact not found".to_string()));
                }

                // Soft delete by setting is_active to false
                ContactRepository::soft_delete(pool, contact_id).await?;

                tracing::info!(
                    "Contact soft deleted successfully: {} {} ({})",
                    contact.first_name,
                    contact.last_name,
                    contact.email
                );

                Ok(())
            }
            None => {
                tracing::warn!("Contact not found for deletion with ID: {}", contact_id);
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

    /// Save custom field values for a contact
    async fn save_custom_fields(
        pool: &PgPool,
        contact_id: Uuid,
        custom_fields: HashMap<String, String>,
    ) -> Result<(), AppError> {
        for (field_name, field_value) in custom_fields {
            // Skip empty values
            if field_value.trim().is_empty() {
                continue;
            }

            // Find the custom field definition
            let custom_field = ContactCustomValueRepository::find_custom_field_by_name(pool, &field_name).await?;

            if let Some(field) = custom_field {
                // Create custom value
                let mut custom_value = ContactCustomValue::new(contact_id, field.id);

                // Set value based on field type
                if let Err(error) = custom_value.set_value(&field.field_type, &field_value) {
                    tracing::warn!("Invalid value for field {}: {}", field_name, error);
                    continue; // Skip invalid values
                }

                // Save to database
                ContactCustomValueRepository::upsert(pool, &custom_value).await?;

                tracing::info!("Saved custom field {} = {} for contact {}", field_name, field_value, contact_id);
            } else {
                tracing::warn!("Custom field '{}' not found, skipping", field_name);
            }
        }
        Ok(())
    }

    /// Patch custom fields (merge semantics - add/update fields, remove if empty)
    async fn patch_custom_fields(
        pool: &PgPool,
        contact_id: Uuid,
        custom_fields: HashMap<String, String>,
    ) -> Result<(), AppError> {
        for (field_name, field_value) in custom_fields {
            // Find the custom field definition
            let custom_field = ContactCustomValueRepository::find_custom_field_by_name(pool, &field_name).await?;

            if let Some(field) = custom_field {
                if field_value.trim().is_empty() {
                    // Remove the custom field value if empty string is provided
                    ContactCustomValueRepository::delete_by_contact_and_field(pool, contact_id, field.id).await?;
                    tracing::info!("Removed custom field {} for contact {}", field_name, contact_id);
                } else {
                    // Create or update custom value
                    let mut custom_value = ContactCustomValue::new(contact_id, field.id);

                    // Set value based on field type
                    if let Err(error) = custom_value.set_value(&field.field_type, &field_value) {
                        tracing::warn!("Invalid value for field {}: {}", field_name, error);
                        continue; // Skip invalid values
                    }

                    // Save to database
                    ContactCustomValueRepository::upsert(pool, &custom_value).await?;

                    tracing::info!("Patched custom field {} = {} for contact {}", field_name, field_value, contact_id);
                }
            } else {
                tracing::warn!("Custom field '{}' not found, skipping", field_name);
            }
        }
        Ok(())
    }

    /// Get custom field values for a contact
    async fn get_custom_fields_for_contact(
        pool: &PgPool,
        contact_id: Uuid,
    ) -> Result<Option<HashMap<String, String>>, AppError> {
        let custom_values = ContactCustomValueRepository::find_by_contact_id(pool, contact_id).await?;

        if custom_values.is_empty() {
            return Ok(None);
        }

        let mut fields_map = HashMap::new();

        // Get all custom fields to map IDs to field names
        let custom_fields = ContactCustomValueRepository::get_contact_custom_fields(pool).await?;
        let field_map: HashMap<Uuid, String> = custom_fields
            .into_iter()
            .map(|f| (f.id, f.field_name))
            .collect();

        for custom_value in custom_values {
            if let Some(field_name) = field_map.get(&custom_value.custom_field_id) {
                if let Some(value) = custom_value.value {
                    fields_map.insert(field_name.clone(), value);
                }
            }
        }

        if fields_map.is_empty() {
            Ok(None)
        } else {
            Ok(Some(fields_map))
        }
    }
}
