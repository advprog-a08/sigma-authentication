use std::error::Error;

use sigma_authentication::app::{GrpcApp, RestApp};
use sigma_authentication::database::setup_db;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let pool = setup_db().await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate!");

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info")),
        )
        .with_target(true)
        .with_level(true)
        .init();

    let pool_ = pool.clone();
    let grpc_task = tokio::spawn(async move {
        let addr = "[::]:50051";
        tracing::info!("Starting gRPC server at {}", addr);
        let app = GrpcApp::default().with_pool(pool_);
        app.run(addr).await.unwrap();
    });

    let pool_ = pool.clone();
    let rest_task = tokio::spawn(async move {
        let addr = "0.0.0.0:8082";
        tracing::info!("Starting REST server at {}", addr);
        let app = RestApp::default().with_pool(pool_);
        app.run(addr).await.unwrap();
    });

    let (grpc_result, rest_result) = tokio::join!(grpc_task, rest_task);

    if let Err(e) = grpc_result { panic!("gRPC task panicked: {:?}", e) }
    if let Err(e) = rest_result { panic!("REST task panicked: {:?}", e) }

    Ok(())
}
