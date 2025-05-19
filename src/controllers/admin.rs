#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket::serde::json::serde_json::json;

    use crate::app::App;
    use crate::database;
    use crate::repository::AdminRepository;

    #[rocket::async_test]
    async fn test_login_success() {
        let test_db = database::setup_test_db().await;

        let admin_repository = AdminRepository::new(test_db.pool.clone());
        admin_repository.create("test@example.com".to_string(), "password123".to_string()).await.unwrap();

        let rocket = App::default().with_pool(test_db.pool).rocket();
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client
            .post("/admin/login")
            .json(&json!({
                "email": "test@example.com",
                "password": "password123"
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_login_unknown_user() {
        let test_db = database::setup_test_db().await;

        let rocket = App::default().with_pool(test_db.pool).rocket();
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client
            .post("/admin/login")
            .json(&json!({
                "email": "test@example.com",
                "password": "password123"
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::BadRequest);
    }
}
