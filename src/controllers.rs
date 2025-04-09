use rocket::{fairing::AdHoc, routes};

mod home;

pub fn route_stage() -> AdHoc {
    AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket.mount("/", routes![home::index, home::login])
    })
}
