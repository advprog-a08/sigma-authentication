pub(crate) mod proto {
    tonic::include_proto!("admin");
}

mod admin_grpc;
mod admin_model;
mod admin_repository;
mod admin_rest;
mod admin_service;

pub use admin_grpc::*;
pub use admin_model::*;
pub use admin_repository::*;
pub use admin_rest::*;
pub use admin_service::*;

#[cfg(test)]
mod tests {
    use tonic::Request;

    use crate::database;
    use crate::token::TokenService;

    use super::*;
    use super::proto::{self, admin_service_server::AdminService as _};

    #[tokio::test]
    async fn test_verify_success() {
        let test_db = database::setup_test_db().await;
        let admin_repository = AdminRepository::new(test_db.pool.clone());

        // first, create admin
        admin_repository.create(
            "test@example.com".to_string(),
            "test".to_string(),
            "HelloWorld123!".to_string()
        ).await.unwrap();

        let admin_service = AdminService::new(admin_repository);
        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());

        // second, create token
        let token = token_service.create_jwt("test@example.com".to_string()).unwrap();

        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        // third, verify the token
        let result = admin_grpc.verify_admin(Request::new(proto::TokenRequest {
            token,
        })).await.unwrap();

        let admin = result.into_inner().admin.unwrap();
        assert_eq!(admin.email, "test@example.com".to_string());
    }

    #[tokio::test]
    async fn test_verify_fail_using_random_jwt() {
        let test_db = database::setup_test_db().await;
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let admin_service = AdminService::new(admin_repository);
        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());

        // first, create the random token
        let token = token_service.create_jwt("asdfadfasdf".to_string()).unwrap();

        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        // second, verify the token
        let result = admin_grpc.verify_admin(Request::new(proto::TokenRequest {
            token,
        })).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), tonic::Code::NotFound);
        assert_eq!(error.message(), "Admin not found".to_string());
    }
}
