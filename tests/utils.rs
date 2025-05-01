use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres as PostgresContainer;

pub struct TestDb {
    pub pool: Pool<Postgres>,
    #[allow(dead_code)]
    _container: ContainerAsync<testcontainers_modules::postgres::Postgres>,
}

pub async fn setup_test_db() -> TestDb {
    let node = PostgresContainer::default()
        .with_tag("15")
        .start()
        .await
        .unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgresql://postgres:postgres@localhost:{}/postgres",
            node.get_host_port_ipv4(5432).await.unwrap()
        ))
        .await
        .unwrap();

    let migrations_path = std::path::Path::new("./migrations");

    sqlx::migrate::Migrator::new(migrations_path)
        .await
        .expect("Could not create migrator")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    TestDb { pool, _container: node }
}
