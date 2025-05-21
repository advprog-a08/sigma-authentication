use std::error::Error;

use sqlx::PgPool;
use tonic::transport::Server;

use crate::admin::{AdminGrpc, AdminRepository, AdminService};
use crate::admin::proto::admin_service_server::AdminServiceServer;
use crate::token::TokenService;

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

        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());

        let admin_grpc = AdminGrpc::new(admin_service, token_service);

        Server::builder()
            .add_service(AdminServiceServer::new(admin_grpc))
            .serve(addr)
            .await?;

        Ok(())
    }
}
