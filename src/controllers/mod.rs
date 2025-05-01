use rocket::routes;
use rocket::fairing::AdHoc;

pub mod home;

pub fn route_stage() -> AdHoc {
    AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket.mount("/", routes![home::index])
    })
}
