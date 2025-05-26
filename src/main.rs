use std::error::Error;

use sigma_authentication::app::{GrpcApp, RestApp};
use sigma_authentication::database::setup_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let pool = setup_db().await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate!");

    let pool_ = pool.clone();
    let grpc_task = tokio::spawn(async move {
        let app = GrpcApp::default().with_pool(pool_);
        app.run("[::1]:50051").await.unwrap();
    });

    let pool_ = pool.clone();
    let rest_task = tokio::spawn(async move {
        let app = RestApp::default().with_pool(pool_);
        app.run("0.0.0.0:8082").await.unwrap();
    });

    let _ = tokio::join!(grpc_task, rest_task);

    Ok(())
}
