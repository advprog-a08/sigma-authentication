use std::error::Error;
use std::sync::Arc;

use axum::routing;
use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tonic::transport::Server;

use crate::admin;
use crate::admin::{AdminGrpc, AdminRepository, AdminService};
use crate::admin::proto::admin_service_server::AdminServiceServer;
use crate::table_session::{TableSessionGrpc, TableSessionRepository, TableSessionService};
use crate::table_session::proto::table_session_service_server::TableSessionServiceServer;
use crate::token::TokenService;

#[derive(Default)]
pub struct GrpcApp {
    pool: Option<PgPool>,
}

impl GrpcApp {
    pub fn with_pool(mut self, pool: PgPool) -> Self {
        self.pool = Some(pool);
        self
    }

    pub async fn run(self, addr: &str) -> Result<(), Box<dyn Error>> {
        let pool = self.pool.expect("`pool` not set!");
        let addr = addr.parse()?;

        let admin_repository = AdminRepository::new(pool.clone());
        let admin_service = AdminService::new(admin_repository);

        let table_session_repository = TableSessionRepository::new(pool.clone());
        let table_session_service = TableSessionService::new(table_session_repository);

        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());

        let admin_grpc = AdminGrpc::new(admin_service, token_service);
        let table_session_grpc = TableSessionGrpc::new(table_session_service);

        Server::builder()
            .add_service(AdminServiceServer::new(admin_grpc))
            .add_service(TableSessionServiceServer::new(table_session_grpc))
            .serve(addr)
            .await?;

        Ok(())
    }
}

#[derive(Default)]
pub struct RestApp {
    pool: Option<PgPool>,
}

#[derive(Clone)]
pub struct RestState {
    pub admin_service: Arc<AdminService>,
    pub token_service: Arc<TokenService>,
}

impl RestApp {
    pub fn with_pool(mut self, pool: PgPool) -> Self {
        self.pool = Some(pool);
        self
    }

    pub async fn run(self, addr: &str) -> Result<(), Box<dyn Error>> {
        async fn hello() -> &'static str {
            "Hello from REST!"
        }

        let pool = self.pool.expect("`pool` not set!");

        let admin_repository = AdminRepository::new(pool.clone());
        let admin_service = AdminService::new(admin_repository);

        let token_service = TokenService::new("asdf".to_string(), "asdf".to_string());

        let state = RestState {
            admin_service: Arc::new(admin_service),
            token_service: Arc::new(token_service),
        };

        let app = Router::new()
            .route("/", routing::get(hello))
            .nest("/admin", admin::router())
            .with_state(state);

        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();

        Ok(())
    }
}
