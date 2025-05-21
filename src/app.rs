use std::error::Error;

use sqlx::PgPool;
use tonic::transport::Server;

use crate::grpc::admin::GrpcAdminService;
use crate::grpc::proto::admin_service_server::AdminServiceServer;
use crate::repository::AdminRepository;
use crate::service::AdminService;

#[derive(Default)]
pub struct App {
    pool: Option<PgPool>,
}

impl App {
    pub fn with_pool(mut self, pool: PgPool) -> Self {
        self.pool = Some(pool);
        self
    }

    pub async fn run(self, addr: &str) -> Result<(), Box<dyn Error>> {
        let pool = self.pool.expect("`pool` not set!");
        let addr = addr.parse()?;

        let admin_repository = AdminRepository::new(pool.clone());
        let admin_service = AdminService::new(admin_repository);

        let grpc_admin_service = GrpcAdminService::new(admin_service);

        Server::builder()
            .add_service(AdminServiceServer::new(grpc_admin_service))
            .serve(addr)
            .await?;

        Ok(())
    }
}
