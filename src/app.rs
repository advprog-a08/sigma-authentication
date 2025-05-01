use rocket::{Build, Rocket};

use crate::controllers::route_stage;
use crate::service::UserService;
use crate::repository::UserRepository;
use crate::registries::StrategyRegistry;

pub struct App {
    strategy_registry: StrategyRegistry,
    user_service: UserService,
}

impl Default for App {
    fn default() -> Self {
        let strategy_registry = StrategyRegistry::default();

        let user_repository = UserRepository::default();
        let user_service = UserService::new(user_repository);

        Self {
            strategy_registry,
            user_service,
        }
    }
}

impl App {
    pub fn rocket(self) -> Rocket<Build> {
        rocket::build()
            .manage(self.strategy_registry)
            .manage(self.user_service)
            .attach(route_stage())
    }

    pub async fn launch(self) -> Result<(), rocket::Error> {
        self.rocket().launch().await.map(|_| ())
    }
}
