use std::error::Error;

use sigma_authentication::app::App;
use sigma_authentication::database::setup_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let pool = setup_db().await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate!");

    let app = App::default().with_pool(pool);
    app.run("[::1]:50051").await?;

    Ok(())
}
