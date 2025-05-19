use rocket::serde::json::Json;
use rocket::post;
use rocket::State;
use rocket::serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::Admin;
use crate::models::AdminCreate;
use crate::service::AdminService;
use crate::service::TokenService;

use super::ApiResponse;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginSuccess {
    pub token: String,
}

#[post("/login", data = "<login_data>")]
pub async fn login(
    login_data: Json<LoginData>,
    admin_service: &State<AdminService>,
    token_service: &State<TokenService>,
) -> ApiResponse<LoginSuccess> {
    let Json(LoginData { email, password }) = login_data;

    match admin_service.authenticate(email.clone(), password).await {
        Ok(_) => {
            match token_service.create_jwt(email) {
                Ok(token) => ApiResponse::success(LoginSuccess { token }),
                Err(_) => ApiResponse::general_error("Failed to create authentication token".to_string()),
            }
        },

        Err(err) => ApiResponse::general_error(err.to_string()),
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateSuccess {
    admin: Admin,
}

#[post("/", data = "<admin_data>")]
pub async fn create(
    admin_data: Json<AdminCreate>,
    admin_service: &State<AdminService>,
) -> ApiResponse<CreateSuccess> {
    if let Err(e) = admin_data.validate() {
        return ApiResponse::validation_error(e);
    }

    let Json(AdminCreate { email, password }) = admin_data;

    match admin_service.find_one(email.clone()).await {
        Ok(None) => {
            match admin_service.register_admin(email, password).await {
                Ok(admin) => ApiResponse::success(CreateSuccess { admin }),
                Err(e) => ApiResponse::general_error(e.to_string()),
            }
        }
        Ok(Some(_)) => ApiResponse::general_error("Email already exists".to_string()),
        Err(e) => ApiResponse::general_error(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket::serde::json::serde_json::json;
    use rocket::serde::json::Value;

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

    #[rocket::async_test]
    async fn test_create_validation() {
        let test_db = database::setup_test_db().await;
        let rocket = App::default().with_pool(test_db.pool.clone()).rocket();
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client
            .post("/admin")
            .json(&json!({
                "email": "testexample.com",
                "password": "1",
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::UnprocessableEntity);

        let body = response.into_json::<Value>().await.expect("valid JSON");

        assert_eq!(
            body["errors"]["email"][0]["message"]
                .as_str()
                .expect("error code is a string"),
            "Email must be valid",
        );

        assert_eq!(
            body["errors"]["password"][0]["message"]
                .as_str()
                .expect("error code is a string"),
            "Password must be at least 8 characters long",
        );

        let admin_repository = AdminRepository::new(test_db.pool.clone());
        assert!(admin_repository.find_one("test@example.com".to_string()).await.unwrap().is_none())
    }

    #[rocket::async_test]
    async fn test_duplicate_email() {
        let test_db = database::setup_test_db().await;

        let admin_repository = AdminRepository::new(test_db.pool.clone());
        admin_repository.create("test@example.com".to_string(), "password123".to_string()).await.unwrap();

        let rocket = App::default().with_pool(test_db.pool).rocket();
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client
            .post("/admin")
            .json(&json!({
                "email": "test@example.com",
                "password": "HelloWorld123!",
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::BadRequest);

        let body = response.into_json::<Value>().await.expect("valid JSON");

        assert_eq!(
            body["message"].as_str().expect("message is a string"),
            "Email already exists",
        );
    }

    #[rocket::async_test]
    async fn test_create_admin() {
        let test_db = database::setup_test_db().await;
        let rocket = App::default().with_pool(test_db.pool.clone()).rocket();
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client
            .post("/admin")
            .json(&json!({
                "email": "test@example.com",
                "password": "HelloWorld123!",
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<Value>().await.expect("valid JSON");

        assert_eq!(
            body["admin"]["email"].as_str().expect("email is a string"),
            "test@example.com",
        );

        let admin_repository = AdminRepository::new(test_db.pool.clone());
        assert!(admin_repository.find_one("test@example.com".to_string()).await.unwrap().is_some())
    }
}
