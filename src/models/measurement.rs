use chrono::{DateTime, Utc};
use super::Status;

pub struct Measurement {
    id: i32,
    chapter_id: i32,
    product_id: i32,
    order: i32,
    description: Option<String>,
    quantity: f64,
    total: f64,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

