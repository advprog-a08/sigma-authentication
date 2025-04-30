use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    pub strategy: String,
    pub email: String,
    pub password: Option<String>,
    pub token: Option<String>,
}

