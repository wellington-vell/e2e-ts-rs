use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
pub type Db = Pool<sqlx::Postgres>;

pub async fn connect() -> Result<Db, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/e2e-ts-rs".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
}

pub async fn migrate(db: &Db) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(db).await?;
    Ok(())
}
