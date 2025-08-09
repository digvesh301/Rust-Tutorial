use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
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
}

impl Contact {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        phone: Option<String>,
        company: Option<String>,
        job_title: Option<String>,
        owner_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            email,
            phone,
            company,
            job_title,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            notes: None,
            lead_source: None,
            lead_status: "new".to_string(),
            owner_id,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
