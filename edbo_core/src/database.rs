use crate::InitSettings;
use sqlx::{Executor, PgPool};
use thiserror::Error;
use url::Url;

pub async fn init(settings: &InitSettings) -> Result<(), DbError> {
    ensure_db_exists(settings).await?;

    Ok(())
}

async fn ensure_db_exists(settings: &InitSettings) -> Result<(), DbError> {
    // Getting needed DB name
    let database_url =
        Url::parse(&settings.database_url).map_err(DbError::FailedParseDbUrl)?;

    let database_name = database_url
        .path()
        .split('/')
        .next_back()
        .ok_or(DbError::DatabaseNameNotFound)?
        .trim();

    if database_name.is_empty() {
        return Err(DbError::DatabaseNameIsEmpty);
    }

    // Connection to admin DB
    let mut admin_url = database_url.clone();
    admin_url.set_path("/postgres");
    let admin_pool = PgPool::connect(admin_url.as_str())
        .await
        .map_err(DbError::FailedConnectToAdmin)?;

    // Is needed database exist?
    let exists: Option<i32> =
        sqlx::query_scalar("SELECT 1 FROM pg_database WHERE datname = $1")
            .bind(database_name)
            .fetch_optional(&admin_pool)
            .await
            .map_err(DbError::FailedRunQuery)?;

    // Creating (or not) core DB
    match exists.is_none() {
        true => {
            log::info!("Core database not exists.");
            admin_pool
                .execute(format!("CREATE DATABASE {database_name};").as_str())
                .await
                .map_err(DbError::FailedCreateCoreDatabase)?;
            log::info!("Core database \"{database_name}\" successfully created!");
        },
        false => {
            log::info!("Core database \"{database_name}\" exists!");
        },
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Failed to parse database URL. {0}")]
    FailedParseDbUrl(url::ParseError),

    #[error("Database name not found in corresponding URL.")]
    DatabaseNameNotFound,

    #[error("Database name is empty.")]
    DatabaseNameIsEmpty,

    #[error("Failed to connect to admin database. {0}")]
    FailedConnectToAdmin(sqlx::Error),

    #[error("Query failed. {0}")]
    FailedRunQuery(sqlx::Error),

    #[error("Failed to create core database. {0}")]
    FailedCreateCoreDatabase(sqlx::Error),
}
