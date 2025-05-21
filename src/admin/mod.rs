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

    use crate::{database, proto::{self, admin_service_server::AdminService as _}};

    use super::*;

    #[tokio::test]
    async fn test_create_admin_success() {
        let test_db = database::setup_test_db().await;
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let admin_service = AdminService::new(admin_repository);
        let admin_grpc = AdminGrpc::new(admin_service);

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
        let admin_grpc = AdminGrpc::new(admin_service);

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

    // TODO: validation
}
