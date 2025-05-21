use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct TableSessionModel {
    pub id: Uuid,
    pub table_id: Uuid,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct TableSessionCreate {
    pub table_id: Uuid,
}
