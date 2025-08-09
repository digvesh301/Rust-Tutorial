// Date utility functions

use chrono::{DateTime, Utc};

/// Format timestamp for API responses
pub fn format_timestamp(timestamp: Option<DateTime<Utc>>) -> String {
    match timestamp {
        Some(dt) => dt.to_rfc3339(),
        None => "Unknown".to_string(),
    }
}

/// Get current UTC timestamp
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}
