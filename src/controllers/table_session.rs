use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{get, post, State};
use uuid::Uuid;

use crate::models::{TableSession, TableSessionCreate};
use crate::service::TableSessionService;

use super::ApiResponse;

pub struct AuthTableSession {
    pub table_session: TableSession,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthTableSession {
    type Error = ApiResponse<()>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(table_session_service) = req.rocket().state::<TableSessionService>() else {
            return Outcome::Error((
                Status::InternalServerError, 
                ApiResponse::general_error("Internal Server Error".to_string()),
            ));
        };

        if let Some(auth) = req.headers().get_one("Authorization") {
            if let Some(token) = auth.strip_prefix("Bearer ") {
                let Ok(token) = Uuid::parse_str(token) else {
                    return Outcome::Error((
                        Status::BadRequest, 
                        ApiResponse::general_error("Internal Server Error".to_string()),
                    ));
                };

                return match table_session_service.find_by_id(token).await {
                    Ok(Some(table_session)) => Outcome::Success(AuthTableSession { table_session }),
                    Ok(None) => Outcome::Error((
                        Status::Unauthorized, 
                        ApiResponse::general_error("Internal Server Error".to_string()),
                    )),
                    Err(_) => Outcome::Error((
                        Status::InternalServerError, 
                        ApiResponse::general_error("Internal Server Error".to_string()),
                    ))
                }
            }
        }

        Outcome::Error((
            Status::Unauthorized, 
            ApiResponse::general_error("Internal Server Error".to_string()),
        ))
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateSessionSuccess {
    pub session_id: String,
}

#[post("/", data = "<session_data>")]
pub async fn create_session(
    session_data: Json<TableSessionCreate>,
    table_session_service: &State<TableSessionService>,
) -> ApiResponse<CreateSessionSuccess> {
    match table_session_service.create_session(session_data.table_id).await {
        Ok(table_session) => {
            let session_id = table_session.id.to_string();
            ApiResponse::success(CreateSessionSuccess { session_id })
        },
        Err(e) => ApiResponse::general_error(e.to_string()),
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetSessionSuccess {
    pub table_session: TableSession,
}

#[get("/")]
pub async fn get_session(
    auth_table_session: AuthTableSession,
) -> ApiResponse<GetSessionSuccess> {
    let table_session = auth_table_session.table_session;
    ApiResponse::success(GetSessionSuccess { table_session })
}

#[cfg(test)]
mod tests {
    use rocket::http::{Header, Status};
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

        body["session_id"].as_str().expect("session_id is a string");
    }

    #[rocket::async_test]
    async fn test_get_table_session_success() {
        let test_db = database::setup_test_db().await;
        let rocket = App::default().with_pool(test_db.pool).rocket();
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let table_id = Uuid::new_v4();
        let response = client
            .post("/table-session")
            .json(&json!({ "table_id": table_id.to_string() }))
            .dispatch()
            .await;

        let body = response.into_json::<Value>().await.expect("valid JSON");
        let session_id = body["session_id"].as_str().expect("session_id is a string");

        let response = client
            .get("/table-session")
            .header(Header::new("Authorization", format!("Bearer {}", session_id)))
            .dispatch()
            .await;

        let body = response.into_json::<Value>().await.expect("valid JSON");
        body["table_session"].as_object().expect("table_session is an object");
    }

    #[rocket::async_test]
    async fn test_get_table_session_fail() {
        let test_db = database::setup_test_db().await;
        let rocket = App::default().with_pool(test_db.pool).rocket();
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let session_id = Uuid::new_v4();

        let response = client
            .get("/table-session")
            .header(Header::new("Authorization", format!("Bearer {}", session_id)))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Unauthorized);

        // TODO: check response body
    }
}
