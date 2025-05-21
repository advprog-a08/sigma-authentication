mod proto {
    tonic::include_proto!("table_session");
}

mod table_session_grpc;
mod table_session_model;
mod table_session_repository;
mod table_session_service;

pub use table_session_grpc::*;
pub use table_session_model::*;
pub use table_session_repository::*;
pub use table_session_service::*;

#[cfg(test)]
mod tests {
    use tonic::Request;
    use uuid::Uuid;

    use crate::database;

    use super::*;
    use super::proto::table_session_service_server::TableSessionService as _;

    #[tokio::test]
    async fn test_create_table_session_success() {
        let test_db = database::setup_test_db().await;
        let table_session_repository = TableSessionRepository::new(test_db.pool.clone());
        let table_session_service = TableSessionService::new(table_session_repository);
        let table_session_grpc = TableSessionGrpc::new(table_session_service);

        let request = Request::new(proto::CreateTableSessionRequest {
            table_id: Uuid::new_v4().to_string(),
        });

        // test by only unwrapping
        table_session_grpc.create_table_session(request).await.unwrap();
    }
}
