use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::proto;

#[derive(Debug, Clone, Serialize)]
pub struct TableSessionModel {
    pub id: Uuid,
    pub table_id: Uuid,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<TableSessionModel> for proto::TableSession {
    fn from(value: TableSessionModel) -> Self {
        Self {
            id: value.id.to_string(),
            table_id: value.table_id.to_string(),
            is_active: value.is_active,
        }
    }
}
