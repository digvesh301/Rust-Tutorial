// Organization domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub country: Option<String>,
    pub timezone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

impl Organization {
    pub fn new(name: String, country: Option<String>, timezone: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            country,
            timezone,
            created_at: Some(Utc::now()),
        }
    }
}
