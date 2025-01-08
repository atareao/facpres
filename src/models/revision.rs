use chrono::{
    DateTime,
    Utc,
};
use super::Status;

pub struct Revision {
    id: i32,
    budget_id: i32,
    number: i32,
    description: Option<String>,
    date: DateTime<Utc>,
    until: DateTime<Utc>,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}


