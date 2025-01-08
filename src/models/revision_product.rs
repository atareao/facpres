use chrono::{DateTime, Utc};
use super::Status;

pub struct RevisionProduct {
    revision_id: i32,
    product_id: i32,
    group: Option<String>,
    description: Option<String>,
    quantity: f64,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
