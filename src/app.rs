use rocket::{Build, Rocket};

use crate::controllers::route_stage;
use crate::service::UserService;
use crate::repository::UserRepository;

pub struct App {
    user_service: UserService,
}

impl Default for App {
    fn default() -> Self {
        let user_repository = UserRepository::default();
        let user_service = UserService::new(user_repository);

        Self {
            user_service,
        }
    }
}

impl App {
    pub fn rocket(self) -> Rocket<Build> {
        rocket::build()
            .manage(self.user_service)
            .attach(route_stage())
    }

    pub async fn launch(self) -> Result<(), rocket::Error> {
        self.rocket().launch().await.map(|_| ())
    }
}
