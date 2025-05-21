pub(crate) mod proto {
    tonic::include_proto!("admin");
}

mod admin_grpc;
mod admin_model;
mod admin_repository;
mod admin_service;

pub use admin_grpc::*;
pub use admin_model::*;
pub use admin_repository::*;
pub use admin_service::*;

#[cfg(test)]
mod tests {
    use tonic::Request;

    use crate::database;
    use crate::token::TokenService;

    use super::*;
    use super::proto::{self, admin_service_server::AdminService as _};

    #[tokio::test]
    async fn test_create_admin_success() {
        let test_db = database::setup_test_db().await;
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let admin_service = AdminService::new(admin_repository);
        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());
        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        let request = Request::new(proto::CreateAdminRequest {
            email: "test@example.com".to_string(),
            password: "HelloWorld123!".to_string(),
        });

        let result = admin_grpc.create_admin(request).await.unwrap();

        let response = result.into_inner();
        assert_eq!(response.admin.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_create_admin_duplicate_email() {
        let test_db = database::setup_test_db().await;
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let admin_service = AdminService::new(admin_repository);
        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());
        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        // create existing admin
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        admin_repository.create("test@example.com".to_string(), "password123".to_string()).await.unwrap();

        // execute request
        let request = Request::new(proto::CreateAdminRequest {
            email: "test@example.com".to_string(),
            password: "HelloWorld123!".to_string(),
        });

        let result = admin_grpc.create_admin(request).await;

        assert!(result.is_err());
        let status = result.unwrap_err();
        assert_eq!(status.code(), tonic::Code::AlreadyExists);
        assert_eq!(status.message(), "Email already exists");
    }

    #[tokio::test]
    async fn test_create_admin_validation() {
        let test_db = database::setup_test_db().await;
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let admin_service = AdminService::new(admin_repository);
        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());
        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        let request = Request::new(proto::CreateAdminRequest {
            email: "testexample.com".to_string(),
            password: "HelloWorld123!".to_string(),
        });

        let result = admin_grpc.create_admin(request).await;

        assert!(result.is_err());
        let status = result.unwrap_err();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert_eq!(status.message(), "Validation failed: email: Email must be valid");
    }

    #[tokio::test]
    async fn test_login_success() {
        let test_db = database::setup_test_db().await;
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let admin_service = AdminService::new(admin_repository);
        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());
        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        let admin_repository = AdminRepository::new(test_db.pool.clone());
        admin_repository.create("test@example.com".to_string(), "password123".to_string()).await.unwrap();

        let request = Request::new(proto::LoginAdminRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        });

        admin_grpc.login_admin(request).await.unwrap();
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        let test_db = database::setup_test_db().await;
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let admin_service = AdminService::new(admin_repository);
        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());
        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        let admin_repository = AdminRepository::new(test_db.pool.clone());
        admin_repository.create("test@example.com".to_string(), "password123".to_string()).await.unwrap();

        let request = Request::new(proto::LoginAdminRequest {
            email: "test@example.com".to_string(),
            password: "password12".to_string(),
        });

        let result = admin_grpc.login_admin(request).await;

        assert!(result.is_err());
        let status = result.unwrap_err();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert_eq!(status.message(), "The provided credentials is incorrect");
    }

    #[tokio::test]
    async fn test_login_unknown_user() {
        let test_db = database::setup_test_db().await;
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let admin_service = AdminService::new(admin_repository);
        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());
        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        let request = Request::new(proto::LoginAdminRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        });

        let result = admin_grpc.login_admin(request).await;

        assert!(result.is_err());
        let status = result.unwrap_err();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert_eq!(status.message(), "The provided credentials is incorrect");
    }
}
