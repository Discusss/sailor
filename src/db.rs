use std::time::Duration;
use sea_orm::*;

pub(super) async fn set_up_db(database_url: String) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(15)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public");

    let db = Database::connect(opt).await?;
    Ok(db)
}