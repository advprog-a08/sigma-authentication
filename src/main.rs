use std::env;

use sigma_authentication::app::App;
use sqlx::postgres::PgPoolOptions;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().expect("Unable to read environment variables!");

    let db_url = env::var("DATABASE_URL")
        .expect("Unable to read DATABASE_URL!");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Unable to connect to DB!");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate!");

    let app = App::default().with_pool(pool);
    app.launch().await
}
