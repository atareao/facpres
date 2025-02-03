use chrono::{DateTime, Utc};
use super::{Unit, Status};

pub struct Product {
    id: i32,
    code: String,
    short_description: String,
    description: Option<String>,
    unit: Unit,
    price: f64,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
