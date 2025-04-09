use sigma_authentication::registries::StrategyRegistry;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let registry = StrategyRegistry::default();

    rocket::build().manage(registry).mount("/", routes![index])
}
