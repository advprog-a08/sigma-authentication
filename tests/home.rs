// use rocket::local::asynchronous::Client;
// use rocket::http::Status;
//
// use sigma_authentication::app::App;
// use sigma_authentication::database::setup_test_db;
//
// #[rocket::async_test]
// async fn test_homepage() {
//     let rocket = App::default().with_pool(setup_test_db().await).rocket();
//     let client = Client::tracked(rocket).await.expect("valid rocket instance");
//
//     let response = client.get("/").dispatch().await;
//     assert_eq!(response.status(), Status::Ok);
//     assert_eq!(response.into_string().await.unwrap(), "Hello, world!");
// }
