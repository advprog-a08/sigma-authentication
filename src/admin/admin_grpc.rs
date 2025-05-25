use tonic::{Request, Response, Status};

use crate::token::TokenService;

use super::AdminService;
use super::proto;

pub struct AdminGrpc {
    admin_service: AdminService,
    token_service: TokenService,
}

impl AdminGrpc {
    pub fn new(admin_service: AdminService, token_service: TokenService) -> Self {
        Self { admin_service, token_service }
    }
}

#[tonic::async_trait]
impl proto::admin_service_server::AdminService for AdminGrpc {
    async fn create_admin(
        &self,
        _: Request<proto::CreateAdminRequest>,
    ) -> Result<Response<proto::AdminResponse>, Status> {
        Err(Status::unimplemented("Not available through gRPC"))
    }

    async fn login_admin(
        &self,
        _: Request<proto::LoginAdminRequest>,
    ) -> Result<Response<proto::TokenResponse>, Status> {
        Err(Status::unimplemented("Not available through gRPC"))
    }

    async fn verify_admin(
        &self,
        request: Request<proto::TokenRequest>,
    ) -> Result<Response<proto::AdminResponse>, Status> {
        let proto::TokenRequest { token } = request.into_inner();

        let claims = self
            .token_service
            .decode_jwt(token)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        
        let admin = self
            .admin_service
            .find_one(claims.sub)
            .await
            .map_err(|e| Status::invalid_argument(e.to_string()))?
            .ok_or_else(|| Status::not_found("Admin not found"))?;

        Ok(Response::new(proto::AdminResponse { admin: Some(admin.into()) }))
    }

    async fn update_admin(
        &self,
        _: Request<proto::UpdateAdminRequest>,
    ) -> Result<Response<proto::AdminResponse>, Status> {
        Err(Status::unimplemented("Not available through gRPC"))
    }

    async fn delete_admin(
        &self,
        _: Request<proto::DeleteAdminRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("Not available through gRPC"))
    }
}
