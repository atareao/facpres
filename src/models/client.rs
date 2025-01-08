use std::collections::HashMap;
use chrono::{
    DateTime,
    Utc,
};
use super::Status;

pub struct Client {
    pub id: i32,
    pub name: String,
    pub surname: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub vars: HashMap<String, String>,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

