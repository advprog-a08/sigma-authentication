use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Json, Request};
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::json;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(errors) => {
                let msg = errors.to_string();
                let body = json!({ "error": msg });
                (StatusCode::BAD_REQUEST, Json(body))
            }
            ServerError::AxumJsonRejection(_) => {
                let body = json!({ "error": self.to_string() });
                (StatusCode::BAD_REQUEST, Json(body))
            }
        }
        .into_response()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(json) = Json::<T>::from_request(req, state).await?;
        json.validate()?;
        Ok(ValidatedJson(json))
    }
}

