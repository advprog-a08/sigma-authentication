use rocket::routes;
use rocket::fairing::AdHoc;

pub mod admin;
pub mod home;

pub fn route_stage() -> AdHoc {
    AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/", routes![home::index])
            .mount("/admin", routes![admin::login, admin::create])
    })
}
