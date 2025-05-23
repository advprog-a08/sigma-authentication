use tonic::{Request, Response, Status};

use crate::token::TokenService;

use super::{AdminService, ValidatedCreateAdminRequest};
use super::proto::{self, LoginAdminRequest};

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
        request: Request<proto::CreateAdminRequest>,
    ) -> Result<Response<proto::AdminResponse>, Status> {
        let create_req = ValidatedCreateAdminRequest::try_from(request.into_inner())?;

        match self.admin_service.find_one(create_req.email.clone()).await {
            Ok(None) => {
                match self
                    .admin_service
                    .register_admin(create_req.email, create_req.password)
                    .await
                {
                    Ok(admin) => {
                        let grpc_admin = proto::Admin { email: admin.email };
                        Ok(Response::new(proto::AdminResponse {
                            admin: Some(grpc_admin),
                        }))
                    }
                    Err(e) => Err(Status::internal(format!("Failed to create admin: {}", e))),
                }
            }
            Ok(Some(_)) => Err(Status::already_exists("Email already exists")),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn login_admin(
        &self,
        request: Request<proto::LoginAdminRequest>,
    ) -> Result<Response<proto::TokenResponse>, Status> {
        let LoginAdminRequest { email, password } = request.into_inner();

        match self.admin_service.authenticate(email.clone(), password).await {
            Ok(_) => {
                match self.token_service.create_jwt(email) {
                    Ok(token) => Ok(Response::new(proto::TokenResponse { token })),
                    Err(_) => Err(Status::invalid_argument("Failed to create authentication token")),
                }
            },

            Err(err) => Err(Status::invalid_argument(err.to_string())),
        }
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
        request: Request<proto::UpdateAdminRequest>
    ) -> Result<Response<proto::AdminResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn delete_admin(
        &self,
        request: Request<proto::DeleteAdminRequest>
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}
