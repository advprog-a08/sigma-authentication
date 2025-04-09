use rocket::{get, post, State};
use rocket::serde::json::Json;
use rocket::http::Status;

use crate::{models::Credentials, registries::StrategyRegistry};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[post("/login", data = "<creds>")]
pub async fn login(
    creds: Json<Credentials>,
    registry: &State<StrategyRegistry>,
) -> Result<String, Status> {
    let strategy = registry.get(&creds.strategy).ok_or(Status::BadRequest)?;

    let user = strategy.authenticate(creds.into_inner()).await
        .map_err(|_| Status::Unauthorized)?;

    Ok(format!("JWT for {}", user.email))
}
