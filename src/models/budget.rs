use chrono::{
    DateTime,
    Utc,
};
use super::Status;

pub struct Budget {
    id: i32,
    name: String,
    description: Option<String>,
    client_id: i32,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

