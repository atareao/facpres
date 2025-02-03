use chrono::{DateTime, Utc};
use super::Status;

pub struct Chapter {
    id: i32,
    revision_id: i32,
    title: String,
    description: Option<String>,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}


