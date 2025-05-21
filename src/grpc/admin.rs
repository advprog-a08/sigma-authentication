use tonic::{Request, Response, Status};

use crate::service::AdminService;

use super::proto::{
    Admin, CreateAdminRequest, CreateAdminResponse,
    admin_service_server::AdminService as AdminServiceGrpc,
};

pub struct GrpcAdminService {
    admin_service: AdminService,
}

impl GrpcAdminService {
    pub fn new(admin_service: AdminService) -> Self {
        Self { admin_service }
    }
}

#[tonic::async_trait]
impl AdminServiceGrpc for GrpcAdminService {
    async fn create_admin(
        &self,
        request: Request<CreateAdminRequest>,
    ) -> Result<Response<CreateAdminResponse>, Status> {
        let create_req = request.into_inner();

        match self.admin_service.find_one(create_req.email.clone()).await {
            Ok(None) => {
                match self
                    .admin_service
                    .register_admin(create_req.email, create_req.password)
                    .await
                {
                    Ok(admin) => {
                        let grpc_admin = Admin { email: admin.email };
                        Ok(Response::new(CreateAdminResponse {
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
}
