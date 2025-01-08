use chrono::{DateTime, Utc};
use super::Status;

pub struct Product {
    id: i32,
    name: String,
    description: Option<String>,
    price: f64,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
