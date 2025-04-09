use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "error", content = "message", crate = "rocket::serde")]
pub enum AuthError {
    InvalidCredentials,
    MissingField(String),
    TokenVerificationFailed,
    UserNotFound,
    InternalError(String),
}
