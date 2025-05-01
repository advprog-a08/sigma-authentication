use rocket::{Build, Rocket};
use sqlx::PgPool;

use crate::controllers::route_stage;
use crate::service::UserService;
use crate::repository::UserRepository;

#[derive(Default)]
pub struct App {
    pool: Option<PgPool>,
}

impl App {
    pub fn rocket(self) -> Rocket<Build> {
        let pool = self.pool.expect("`pool` not set!");

        let user_repository = UserRepository::new(pool.clone());
        let user_service = UserService::new(user_repository);

        rocket::build()
            .manage(user_service)
            .attach(route_stage())
    }

    pub fn with_pool(mut self, pool: PgPool) -> Self {
        self.pool = Some(pool);
        self
    }

    pub async fn launch(self) -> Result<(), rocket::Error> {
        self.rocket().launch().await.map(|_| ())
    }
}
