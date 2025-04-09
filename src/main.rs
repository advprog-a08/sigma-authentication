#[macro_use] extern crate rocket;

use sigma_authentication::controllers::route_stage;
use sigma_authentication::registries::StrategyRegistry;

#[launch]
fn rocket() -> _ {
    let registry = StrategyRegistry::default();

    rocket::build().manage(registry).attach(route_stage())
}
