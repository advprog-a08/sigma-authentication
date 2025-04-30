use rocket::{fairing::AdHoc, routes};

pub mod home;

pub fn route_stage() -> AdHoc {
    AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket.mount("/", routes![home::index, home::login])
    })
}
