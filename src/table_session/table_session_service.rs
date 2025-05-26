use thiserror::Error;
use uuid::Uuid;

use super::{TableSessionModel, TableSessionRepository, TableSessionRepositoryError};

#[derive(Error, Debug)]
pub enum TableSessionServiceError {
    #[error("{0}")]
    Repository(#[from] TableSessionRepositoryError),
}

pub struct TableSessionService {
    repo: TableSessionRepository,
}

impl TableSessionService {
    pub fn new(repo: TableSessionRepository) -> Self {
        Self { repo }
    }

    pub async fn create_session(
        &self,
        table_id: Uuid,
        order_id: Uuid,
    ) -> Result<TableSessionModel, TableSessionServiceError> {
        Ok(self.repo.create(table_id, order_id).await?)
    }

    pub async fn deactivate_session(
        &self,
        table_id: Uuid,
    ) -> Result<Option<TableSessionModel>, TableSessionServiceError> {
        Ok(self.repo.deactivate(table_id).await?)
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<TableSessionModel>, TableSessionServiceError> {
        Ok(self.repo.find_by_id(id).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::{TableSessionRepository, TableSessionService};
    use crate::database::setup_test_db;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_session() {
        let test_db = setup_test_db().await;
        let table_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();

        let repo = TableSessionRepository::new(test_db.pool);
        let service = TableSessionService::new(repo);

        let result = service.create_session(table_id, order_id).await.unwrap();

        assert_eq!(result.table_id, table_id);
        assert!(result.is_active);
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let test_db = setup_test_db().await;
        let table_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();

        let repo = TableSessionRepository::new(test_db.pool);
        let table_session = repo.create(table_id, order_id).await.unwrap();

        let service = TableSessionService::new(repo);
        let result = service.find_by_id(table_session.id).await.unwrap();
        let Some(result) = result else { panic!() };

        assert_eq!(result.id, table_session.id);
        assert_eq!(result.table_id, table_id);
    }

    #[tokio::test]
    async fn test_deactivate_session() {
        let test_db = setup_test_db().await;
        let table_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();

        let repo = TableSessionRepository::new(test_db.pool);

        // First create a session
        let created_session = repo.create(table_id, order_id).await.unwrap();
        assert!(created_session.is_active);

        // Now use the service to deactivate it
        let service = TableSessionService::new(repo);
        let deactivated_session = service
            .deactivate_session(created_session.id)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(deactivated_session.id, created_session.id);
        assert!(!deactivated_session.is_active);
    }

    #[tokio::test]
    async fn test_deactivate_nonexistent_session() {
        let test_db = setup_test_db().await;
        let random_id = Uuid::new_v4();
        let repo = TableSessionRepository::new(test_db.pool);
        let service = TableSessionService::new(repo);

        let result = service.deactivate_session(random_id).await;

        let result = result.unwrap();
        assert!(result.is_none());
    }
}
