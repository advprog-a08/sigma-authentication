use rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub email: String,
    pub password: String,
}
