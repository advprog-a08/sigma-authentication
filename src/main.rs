#[macro_use] extern crate rocket;

use sigma_authentication::controllers::route_stage;
use sigma_authentication::registries::StrategyRegistry;
use sigma_authentication::repository::UserRepository;
use sigma_authentication::service::UserService;

#[launch]
fn rocket() -> _ {
    let registry = StrategyRegistry::default();

    let user_repository = UserRepository::default();
    let user_service = UserService::new(user_repository);

    rocket::build()
        .manage(registry)
        .manage(user_service)
        .attach(route_stage())
}
