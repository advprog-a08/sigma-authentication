use sqlx::{PgPool, query_as};
use thiserror::Error;
use uuid::Uuid;

use super::TableSession;

#[derive(Error, Debug)]
pub enum TableSessionRepositoryError {
    #[error("An error occurred with the database")]
    Database(#[from] sqlx::Error),
}

pub struct TableSessionRepository {
    pool: PgPool,
}

impl TableSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        table_id: Uuid,
    ) -> Result<TableSession, TableSessionRepositoryError> {
        Ok(query_as!(
            TableSession,
            r#"
            INSERT INTO table_sessions (table_id)
            VALUES ($1)
            RETURNING id, table_id, is_active, created_at
            "#,
            table_id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<TableSession>, TableSessionRepositoryError> {
        Ok(query_as!(
            TableSession,
            r#"
            SELECT *
            FROM table_sessions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<TableSession, TableSessionRepositoryError> {
        Ok(query_as!(
            TableSession,
            r#"
            UPDATE table_sessions
            SET is_active = FALSE
            WHERE id = $1
            RETURNING id, table_id, is_active, created_at
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::setup_test_db;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_table_session() {
        let test_db = setup_test_db().await;
        let tsr = TableSessionRepository::new(test_db.pool);
        let table_id = Uuid::new_v4();

        let session = tsr.create(table_id).await.unwrap();

        assert_eq!(session.table_id, table_id);
        assert!(session.is_active);
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let test_db = setup_test_db().await;
        let tsr = TableSessionRepository::new(test_db.pool);
        let table_id = Uuid::new_v4();

        let created_session = tsr.create(table_id).await.unwrap();
        let Some(found_session) = tsr.find_by_id(created_session.id).await.unwrap() else {
            panic!()
        };

        assert_eq!(found_session.id, created_session.id);
        assert_eq!(found_session.table_id, table_id);
        assert!(found_session.is_active);
    }

    #[tokio::test]
    async fn test_deactivate_session() {
        let test_db = setup_test_db().await;
        let tsr = TableSessionRepository::new(test_db.pool);
        let table_id = Uuid::new_v4();

        let created_session = tsr.create(table_id).await.unwrap();
        assert!(created_session.is_active);

        let deactivated_session = tsr.deactivate(created_session.id).await.unwrap();
        assert_eq!(deactivated_session.id, created_session.id);
        assert!(!deactivated_session.is_active);

        // Verify the session is indeed deactivated in the database
        let Some(found_session) = tsr.find_by_id(created_session.id).await.unwrap() else {
            panic!()
        };
        assert!(!found_session.is_active);
    }

    #[tokio::test]
    async fn test_find_nonexistent_session() {
        let test_db = setup_test_db().await;
        let tsr = TableSessionRepository::new(test_db.pool);
        let random_id = Uuid::new_v4();

        let result = tsr.find_by_id(random_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_deactivate_nonexistent_session() {
        let test_db = setup_test_db().await;
        let tsr = TableSessionRepository::new(test_db.pool);
        let random_id = Uuid::new_v4();

        let result = tsr.deactivate(random_id).await;
        assert!(result.is_err());

        match result {
            Err(TableSessionRepositoryError::Database(e)) => {
                assert!(e.to_string().contains("no rows returned"));
            }
            _ => panic!("Expected Database error with no rows returned"),
        }
    }
}
