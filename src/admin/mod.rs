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
            name: "test".to_string(),
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
        admin_repository.create(
            "test@example.com".to_string(),
            "test".to_string(),
            "password123".to_string()
        ).await.unwrap();

        // execute request
        let request = Request::new(proto::CreateAdminRequest {
            email: "test@example.com".to_string(),
            name: "test".to_string(),
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
            name: "test".to_string(),
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
        admin_repository.create(
            "test@example.com".to_string(),
            "test".to_string(),
            "password123".to_string()
        ).await.unwrap();

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
        admin_repository.create(
            "test@example.com".to_string(),
            "test".to_string(),
            "password123".to_string()
        ).await.unwrap();

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

    #[tokio::test]
    async fn test_update_one_success() {
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
        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        // second, create token
        let result = admin_grpc.login_admin(Request::new(proto::LoginAdminRequest {
            email: "test@example.com".to_string(),
            password: "HelloWorld123!".to_string(),
        })).await.unwrap();

        let token = result.into_inner().token;

        // third, update the admin
        let result = admin_grpc.update_admin(Request::new(proto::UpdateAdminRequest {
            token,
            new_name: "test123".to_string(),
            new_password: "test123".to_string(),
        })).await.unwrap();

        // fourth, check the admin
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let db_result = admin_repository.find_one("test@example.com".to_string()).await.unwrap();
        assert_eq!(db_result.unwrap().name, "test123".to_string());
        assert_eq!(result.into_inner().admin.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_delete_one_success() {
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
        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        // second, create token
        let result = admin_grpc.login_admin(Request::new(proto::LoginAdminRequest {
            email: "test@example.com".to_string(),
            password: "HelloWorld123!".to_string(),
        })).await.unwrap();

        let token = result.into_inner().token;

        // third, delete the admin
        admin_grpc.delete_admin(Request::new(proto::DeleteAdminRequest {
            token,
        })).await.unwrap();

        // fourth, check the admin
        let admin_repository = AdminRepository::new(test_db.pool.clone());
        let db_result = admin_repository.find_one("test@example.com".to_string()).await.unwrap();
        assert!(db_result.is_none());
    }

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
        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        // second, create token
        let result = admin_grpc.login_admin(Request::new(proto::LoginAdminRequest {
            email: "test@example.com".to_string(),
            password: "HelloWorld123!".to_string(),
        })).await.unwrap();

        let token = result.into_inner().token;

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
