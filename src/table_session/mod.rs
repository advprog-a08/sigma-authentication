pub mod proto {
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
    use std::str::FromStr;

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

        let request = Request::new(proto::TableIdRequest {
            table_id: Uuid::new_v4().to_string(),
            order_id: Uuid::new_v4().to_string(),
        });

        // test by only unwrapping
        let response = table_session_grpc.create_table_session(request).await.unwrap();

        let table_session_repository = TableSessionRepository::new(test_db.pool.clone());
        let session_id = Uuid::from_str(&response.into_inner().table_session.unwrap().id).unwrap();
        let table_session = table_session_repository.find_by_id(session_id).await.unwrap();
        assert!(table_session.is_some());
    }

    #[tokio::test]
    async fn test_verify_table_session_success() {
        let test_db = database::setup_test_db().await;
        let table_session_repository = TableSessionRepository::new(test_db.pool.clone());
        let table_session_service = TableSessionService::new(table_session_repository);
        let table_session_grpc = TableSessionGrpc::new(table_session_service);

        let table_id = Uuid::new_v4().to_string();
        let order_id = Uuid::new_v4().to_string();

        let request = Request::new(proto::TableIdRequest {
            table_id: table_id.clone(),
            order_id: order_id.clone(),
        });

        let response = table_session_grpc.create_table_session(request).await.unwrap();

        let session_id = response.into_inner().table_session.unwrap().id;
        let request = Request::new(proto::SessionIdRequest { session_id });
        let response = table_session_grpc.verify_table_session(request).await.unwrap();

        let table_session = response.into_inner().table_session.unwrap();
        assert_eq!(table_session.table_id, table_id);
    }

    #[tokio::test]
    async fn test_verify_table_session_fail() {
        let test_db = database::setup_test_db().await;
        let table_session_repository = TableSessionRepository::new(test_db.pool.clone());
        let table_session_service = TableSessionService::new(table_session_repository);
        let table_session_grpc = TableSessionGrpc::new(table_session_service);

        let session_id = Uuid::new_v4().to_string();
        let request = Request::new(proto::SessionIdRequest { session_id });
        let response = table_session_grpc.verify_table_session(request).await;

        assert!(response.is_err());
        let status = response.unwrap_err();
        assert_eq!(status.code(), tonic::Code::NotFound);
        assert_eq!(status.message(), "Table Session not found");
    }

    #[tokio::test]
    async fn test_deactivate_session() {
        let test_db = database::setup_test_db().await;
        let table_session_repository = TableSessionRepository::new(test_db.pool.clone());
        let table_session_service = TableSessionService::new(table_session_repository);
        let table_session_grpc = TableSessionGrpc::new(table_session_service);

        // first, create session
        let response = table_session_grpc.create_table_session(
            Request::new(proto::TableIdRequest {
                table_id: Uuid::new_v4().to_string(),
                order_id: Uuid::new_v4().to_string(),
            })
        ).await.unwrap();

        let session_id = response.into_inner().table_session.unwrap().id;

        // second, deactivate session
        let response = table_session_grpc.set_is_active_to_table_session(
            Request::new(proto::IsActiveRequest {
                id: session_id.clone(),
                value: true,
            })
        ).await.unwrap();

        // verify is_active is false
        let session = response.into_inner().table_session.unwrap();
        assert_eq!(session.is_active, false);

        // verify again is_active is false but in database
        let table_session_repository = TableSessionRepository::new(test_db.pool.clone());
        let session_id = Uuid::from_str(&session_id).unwrap();
        let table_session = table_session_repository.find_by_id(session_id).await.unwrap().unwrap();
        assert_eq!(table_session.is_active, false);
    }

    #[tokio::test]
    async fn test_set_checkout_id() {
        let test_db = database::setup_test_db().await;
        let table_session_repository = TableSessionRepository::new(test_db.pool.clone());
        let table_session_service = TableSessionService::new(table_session_repository);
        let table_session_grpc = TableSessionGrpc::new(table_session_service);

        // first, create session
        let response = table_session_grpc.create_table_session(
            Request::new(proto::TableIdRequest {
                table_id: Uuid::new_v4().to_string(),
                order_id: Uuid::new_v4().to_string(),
            })
        ).await.unwrap();

        let session = response.into_inner().table_session.unwrap();
        assert!(session.checkout_id.is_none());

        // second, set the checkout id
        let checkout_id = Uuid::new_v4();
        table_session_grpc.set_checkout_id_to_table_session(
            Request::new(proto::CheckoutIdRequest {
                id: session.id.clone(),
                checkout_id: checkout_id.to_string(),
            })
        ).await.unwrap();

        // third assert
        let table_session_repository = TableSessionRepository::new(test_db.pool.clone());
        let db_response = table_session_repository
            .find_by_id(Uuid::from_str(&session.id).unwrap())
            .await
            .unwrap();

        assert_eq!(db_response.unwrap().checkout_id, Some(checkout_id));
    }
}
