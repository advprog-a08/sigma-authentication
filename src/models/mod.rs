use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TableSession {
    pub id: Uuid,
    pub table_id: Uuid,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
