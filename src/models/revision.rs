use chrono::{DateTime, Utc};
use super::Status;

pub struct Revision {
    id: i32,
    budget_id: i32,
    number: String,
    description: Option<String>,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
