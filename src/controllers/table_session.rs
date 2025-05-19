use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{post, State};

use crate::models::TableSessionCreate;
use crate::service::{TableSessionService, TokenService};

use super::ApiResponse;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateSessionSuccess {
    pub token: String,
}

#[post("/", data = "<session_data>")]
pub async fn create_session(
    session_data: Json<TableSessionCreate>,
    table_session_service: &State<TableSessionService>,
    token_service: &State<TokenService>,
) -> ApiResponse<CreateSessionSuccess> {
    match table_session_service.create_session(session_data.table_id).await {
        Ok(table_session) => {
            match token_service.create_jwt(table_session.id.to_string()) {
                Ok(token) => ApiResponse::success(CreateSessionSuccess { token }),
                Err(_) => ApiResponse::general_error("Failed to create authentication token".to_string()),
            }
        },
        Err(e) => ApiResponse::general_error(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket::serde::json::serde_json::json;
    use rocket::serde::json::Value;
    use uuid::Uuid;

    use crate::database;
    use crate::app::App;

    #[rocket::async_test]
    async fn test_create_table_session_success() {
        let test_db = database::setup_test_db().await;
        let rocket = App::default().with_pool(test_db.pool).rocket();
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let table_id = Uuid::new_v4();
        let response = client
            .post("/table-session")
            .json(&json!({ "table_id": table_id.to_string() }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<Value>().await.expect("valid JSON");

        body["token"].as_str().expect("token is a string");
    }
}
