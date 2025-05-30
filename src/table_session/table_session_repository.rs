use sqlx::{PgPool, query_as};
use thiserror::Error;
use uuid::Uuid;

use super::TableSessionModel;

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
        order_id: Uuid,
    ) -> Result<TableSessionModel, TableSessionRepositoryError> {
        Ok(query_as!(
            TableSessionModel,
            r#"
            INSERT INTO table_sessions (table_id, order_id)
            VALUES ($1, $2)
            RETURNING id, table_id, order_id, checkout_id, is_active, created_at
            "#,
            table_id,
            order_id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<TableSessionModel>, TableSessionRepositoryError> {
        Ok(query_as!(
            TableSessionModel,
            r#"
            SELECT id, table_id, order_id, checkout_id, is_active, created_at
            FROM table_sessions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn deactivate(
        &self,
        id: Uuid,
    ) -> Result<Option<TableSessionModel>, TableSessionRepositoryError> {
        Ok(query_as!(
            TableSessionModel,
            r#"
            UPDATE table_sessions
            SET is_active = FALSE
            WHERE id = $1
            RETURNING id, table_id, order_id, checkout_id, is_active, created_at
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn set_checkout_id(
        &self,
        id: Uuid,
        checkout_id: Option<Uuid>,
    ) -> Result<Option<TableSessionModel>, TableSessionRepositoryError> {
        Ok(query_as!(
            TableSessionModel,
            r#"
            UPDATE table_sessions
            SET checkout_id = $2
            WHERE id = $1
            RETURNING id, table_id, order_id, checkout_id, is_active, created_at
            "#,
            id,
            checkout_id
        )
        .fetch_optional(&self.pool)
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
        let order_id = Uuid::new_v4();

        let session = tsr.create(table_id, order_id).await.unwrap();

        assert_eq!(session.table_id, table_id);
        assert!(session.is_active);
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let test_db = setup_test_db().await;
        let tsr = TableSessionRepository::new(test_db.pool);
        let table_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();

        let created_session = tsr.create(table_id, order_id).await.unwrap();
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
        let order_id = Uuid::new_v4();

        let created_session = tsr.create(table_id, order_id).await.unwrap();
        assert!(created_session.is_active);

        let deactivated_session = tsr.deactivate(created_session.id).await.unwrap().unwrap();
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

        let result = tsr.deactivate(random_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_set_checkout_id() {
        let test_db = setup_test_db().await;
        let tsr = TableSessionRepository::new(test_db.pool);

        let table_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();

        // first create the session
        let session = tsr.create(table_id, order_id).await.unwrap();
        assert!(session.checkout_id.is_none());

        // second set the checkout_id
        let checkout_id = Uuid::new_v4();
        let session = tsr.set_checkout_id(session.id, Some(checkout_id)).await.unwrap();

        // third assert
        assert_eq!(session.unwrap().checkout_id.unwrap(), checkout_id);
    }

    #[tokio::test]
    async fn test_unset_checkout_id() {
        let test_db = setup_test_db().await;
        let tsr = TableSessionRepository::new(test_db.pool);

        let table_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();

        // first create the session
        let session = tsr.create(table_id, order_id).await.unwrap();
        assert!(session.checkout_id.is_none());

        // second set the checkout_id
        let checkout_id = Uuid::new_v4();
        tsr.set_checkout_id(session.id, Some(checkout_id)).await.unwrap();

        // third unset the checkout_id
        let session = tsr.set_checkout_id(session.id, None).await.unwrap();

        // third assert
        assert!(session.unwrap().checkout_id.is_none());
    }
}
