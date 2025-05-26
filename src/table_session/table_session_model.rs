use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::proto;

#[derive(Debug, Clone, Serialize)]
pub struct TableSessionModel {
    pub id: Uuid,
    pub table_id: Uuid,
    pub order_id: Uuid,
    pub checkout_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<TableSessionModel> for proto::TableSession {
    fn from(value: TableSessionModel) -> Self {
        Self {
            id: value.id.to_string(),
            table_id: value.table_id.to_string(),
            order_id: value.order_id.to_string(),
            checkout_id: value
                .checkout_id
                .map(|u| Uuid::to_string(&u))
                .unwrap_or(String::new()),
            is_active: value.is_active,
        }
    }
}
