use crate::InitSettings;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
use thiserror::Error;
use url::Url;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn init(settings: &InitSettings) -> Result<Self, DbError> {
        let database_url = Self::ensure_db_exists(settings).await?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await
            .map_err(DbError::FailedConnectToDatabase)?;

        log::info!("Database connection established.");

        sqlx::migrate!().run(&pool).await?;

        log::info!("Database migration finished.");

        Ok(Self { pool })
    }

    async fn ensure_db_exists(settings: &InitSettings) -> Result<Url, DbError> {
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

        Ok(database_url)
    }
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database name not found in corresponding URL.")]
    DatabaseNameNotFound,

    #[error("Database name is empty.")]
    DatabaseNameIsEmpty,

    #[error("Failed to parse database URL. {0}")]
    FailedParseDbUrl(url::ParseError),

    #[error("Failed to connect to admin database. {0}")]
    FailedConnectToAdmin(sqlx::Error),

    #[error("Failed to connect to the core database. {0}")]
    FailedConnectToDatabase(sqlx::Error),

    #[error("Failed to run database migrations. {0}")]
    FailedRunMigration(#[from] sqlx::migrate::MigrateError),

    #[error("Query failed. {0}")]
    FailedRunQuery(sqlx::Error),

    #[error("Failed to create core database. {0}")]
    FailedCreateCoreDatabase(sqlx::Error),
}
