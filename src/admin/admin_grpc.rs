use tonic::{Request, Response, Status};

use super::AdminService;

use crate::proto;

pub struct AdminGrpc {
    admin_service: AdminService,
}

impl AdminGrpc {
    pub fn new(admin_service: AdminService) -> Self {
        Self { admin_service }
    }
}

#[tonic::async_trait]
impl proto::admin_service_server::AdminService for AdminGrpc {
    async fn create_admin(
        &self,
        request: Request<proto::CreateAdminRequest>,
    ) -> Result<Response<proto::CreateAdminResponse>, Status> {
        let create_req = request.into_inner();

        match self.admin_service.find_one(create_req.email.clone()).await {
            Ok(None) => {
                match self
                    .admin_service
                    .register_admin(create_req.email, create_req.password)
                    .await
                {
                    Ok(admin) => {
                        let grpc_admin = proto::Admin { email: admin.email };
                        Ok(Response::new(proto::CreateAdminResponse {
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
