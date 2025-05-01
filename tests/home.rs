use rocket::local::asynchronous::Client;
use rocket::http::Status;
use sqlx::PgPool;

use sigma_authentication::app::App;

#[sqlx::test]
async fn test_homepage(pool: PgPool) {
    let rocket = App::default().with_pool(pool).rocket();
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client.get("/").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Hello, world!");
}
