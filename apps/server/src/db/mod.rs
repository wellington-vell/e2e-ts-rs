use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
pub type Db = Pool<sqlx::Postgres>;

pub async fn connect() -> Result<Db, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
}

pub async fn migrate(db: &Db) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(db).await?;
    Ok(())
}
