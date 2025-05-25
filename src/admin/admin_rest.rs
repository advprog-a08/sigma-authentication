use axum::Router;
use axum::routing;
use axum::response::{IntoResponse, Response};
use axum::extract::{Json, State};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use hyper::StatusCode;
use serde_json::json;

use crate::app::RestState;
use crate::utils::ValidatedJson;

use super::LoginAdminRequest;
use super::ValidatedCreateAdminRequest;
use super::ValidatedUpdateAdminRequest;

pub fn router() -> Router<RestState> {
    Router::new()
        .route("/login", routing::post(login_handler))
        .route("/", routing::post(create_admin_handler))
        .route("/", routing::get(read_admin_handler))
        .route("/", routing::put(update_admin_handler))
        .route("/", routing::delete(delete_admin_handler))
}

pub async fn login_handler(
    State(RestState { admin_service, token_service }): State<RestState>,
    Json(LoginAdminRequest { email, password }): Json<LoginAdminRequest>,
) -> Response {
    match admin_service.authenticate(email.clone(), password).await {
        Ok(_) => {
            match token_service.create_jwt(email) {
                Ok(token) => {
                    (StatusCode::OK, Json(json!({ "token": token }))).into_response()
                }
                Err(_) => {
                    (StatusCode::BAD_REQUEST, Json(json!({ "message": "Failed to create authentication token" }))).into_response()
                },
            }
        },

        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "message": err.to_string() }))).into_response()
    }
}

pub async fn create_admin_handler(
    State(RestState { admin_service, .. }): State<RestState>,
    ValidatedJson(data): ValidatedJson<ValidatedCreateAdminRequest>,
) -> Response {
    match admin_service.find_one(data.email.clone()).await {
        Ok(None) => {
            match admin_service
                .register_admin(data.email, data.name, data.password)
                .await
            {
                Ok(admin) => (StatusCode::OK, Json(admin)).into_response(),
                Err(e) => (StatusCode::NOT_FOUND, format!("Failed to create admin: {}", e)).into_response(),
            }
        }
        Ok(Some(_)) => (StatusCode::BAD_REQUEST, "Email already exists".to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)).into_response(),
    }
}

pub async fn read_admin_handler(
    State(RestState { admin_service, token_service }): State<RestState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Response, Response> {
    let claims = token_service
        .decode_jwt(bearer.token().to_string())
        .map_err(|_| (StatusCode::UNAUTHORIZED, format!("Unauthenticated")).into_response())?;
    
    let admin = admin_service
        .find_one(claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    Ok((StatusCode::OK, Json(admin)).into_response())
}

pub async fn update_admin_handler(
    State(RestState { admin_service, token_service }): State<RestState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    ValidatedJson(data): ValidatedJson<ValidatedUpdateAdminRequest>,
) -> Result<Response, Response> {
    let claims = token_service
        .decode_jwt(bearer.token().to_string())
        .map_err(|_| (StatusCode::UNAUTHORIZED, format!("Unauthenticated")).into_response())?;
    
    let admin = admin_service
        .find_one(claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    let admin = admin_service
        .update_one(admin.email, data.new_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    Ok((StatusCode::OK, Json(admin)).into_response())
}

pub async fn delete_admin_handler(
    State(RestState { admin_service, token_service }): State<RestState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Response, Response> {
    let claims = token_service
        .decode_jwt(bearer.token().to_string())
        .map_err(|_| (StatusCode::UNAUTHORIZED, format!("Unauthenticated")).into_response())?;
    
    let admin = admin_service
        .find_one(claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    admin_service
        .delete_one(admin.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(StatusCode::OK.into_response())
}
