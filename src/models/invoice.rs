use chrono::{
    DateTime,
    Utc,
};
use super::Status;

pub struct Invoice {
    id: i32,
    number: String,
    description: Option<String>,
    client_id: i32,
    date: DateTime<Utc>,
    status: Status,
    total: f64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}


