use rocket::{Build, Rocket};
use sqlx::PgPool;

use crate::controllers::route_stage;
use crate::service::{AdminService, TokenService};
use crate::repository::AdminRepository;

const SERVICE_NAME: &str = "sigma-authentication";
const SECRET_KEY: &str = "asdf";

#[derive(Default)]
pub struct App {
    pool: Option<PgPool>,
}

impl App {
    pub fn rocket(self) -> Rocket<Build> {
        let pool = self.pool.expect("`pool` not set!");

        let admin_repository = AdminRepository::new(pool.clone());
        let admin_service = AdminService::new(admin_repository);

        let token_service = TokenService::new(SERVICE_NAME.to_string(), SECRET_KEY.to_string());

        rocket::build()
            .manage(admin_service)
            .manage(token_service)
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
