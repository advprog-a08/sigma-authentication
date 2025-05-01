use sigma_authentication::database::setup_db;
use sigma_authentication::app::App;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = dotenvy::dotenv();
    let pool = setup_db().await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate!");

    let app = App::default().with_pool(pool);
    app.launch().await
}
