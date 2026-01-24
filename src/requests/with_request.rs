use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateWithRequest {
    pub from: String,
    pub to: String,
    pub played_on: String,
    pub order: u8,
}

impl CreateWithRequest {
    pub fn new(from: &str, to: &str, played_on: &DateTime<Utc>, order: u8) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            played_on: played_on.to_rfc3339(),
            order,
        }
    }
}
