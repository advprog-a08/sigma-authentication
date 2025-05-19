use rocket::serde::json::Json;
use rocket::Responder;
use rocket::post;
use rocket::State;
use rocket::serde::{Deserialize, Serialize};
use validator::Validate;
use validator::ValidationErrors;

use crate::models::AdminCreate;
use crate::service::AdminService;
use crate::service::TokenService;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessResponse {
    pub token: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Responder)]
pub enum LoginResponse {
    #[response(status = 200)]
    Success(Json<SuccessResponse>),

    #[response(status = 400)]
    Error(Json<ErrorResponse>),
}

#[post("/login", data = "<login_data>")]
pub async fn login(
    login_data: Json<LoginData>,
    admin_service: &State<AdminService>,
    token_service: &State<TokenService>,
) -> LoginResponse {
    let Json(LoginData { email, password }) = login_data;

    match admin_service.authenticate(email.clone(), password).await {
        Ok(_) => {
            match token_service.create_jwt(email) {
                Ok(token) => LoginResponse::Success(Json(SuccessResponse { token })),
                Err(_) => LoginResponse::Error(Json(ErrorResponse {
                    message: "Failed to create authentication token".to_string()
                })),
            }
        },

        Err(err) => LoginResponse::Error(Json(ErrorResponse {
            message: err.to_string(),
        })),
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ValidationErrorResponse {
    pub errors: ValidationErrors,
}

#[derive(Responder)]
pub enum CreateResponse {
    #[response(status = 200)]
    Success(Json<SuccessResponse>),

    #[response(status = 422)]
    ValidationError(Json<ValidationErrorResponse>),
}

#[post("/", data = "<admin_data>")]
pub async fn create(
    admin_data: Json<AdminCreate>,
) -> CreateResponse {
    if let Err(e) = admin_data.validate() {
        return CreateResponse::ValidationError(Json(ValidationErrorResponse { errors: e }))
    }

    CreateResponse::Success(Json(SuccessResponse { token: "".to_string() }))
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
        let rocket = App::default().with_pool(test_db.pool).rocket();
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
        println!("{body:?}");

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
    }
}
