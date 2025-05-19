use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{routes, Responder};
use rocket::fairing::AdHoc;
use validator::ValidationErrors;

pub mod admin;
pub mod home;
pub mod table_session;

pub fn route_stage() -> AdHoc {
    AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/", routes![home::index])
            .mount("/admin", routes![admin::login, admin::create])
            .mount("/table-session", routes![table_session::create_session, table_session::get_session])
    })
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneralErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ValidationErrorResponse {
    pub errors: ValidationErrors,
}

#[derive(Debug, Responder)]
pub enum ApiResponse<T> {
    #[response(status = 200)]
    Success(Json<T>),

    #[response(status = 400)]
    GeneralError(Json<GeneralErrorResponse>),

    #[response(status = 422)]
    ValidationError(Json<ValidationErrorResponse>),
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self::Success(Json(data))
    }

    pub fn general_error(message: String) -> Self {
        Self::GeneralError(Json(GeneralErrorResponse { message }))
    }

    pub fn validation_error(errors: ValidationErrors) -> Self {
        Self::ValidationError(Json(ValidationErrorResponse { errors }))
    }
}
