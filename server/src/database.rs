use crate::settings::RuntimeSettings;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres};
use thiserror::Error;

#[derive(Debug)]
pub struct Database {
    pub pool: PgPool,
}

const MAX_CONNECTIONS: u32 = 5;

impl Database {
    pub async fn init(settings: &RuntimeSettings) -> Result<Self, DbError> {
        let url = &settings.database_url;

        let is_db_exists_at_initialization = Postgres::database_exists(url)
            .await
            .map_err(DbError::ExistsOrNotValidation)?;

        if is_db_exists_at_initialization {
            log::info!("Database exists.");
        } else {
            Self::create_database(url).await?;
        }

        let pool = PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(url)
            .await
            .map_err(DbError::Connection)?;

        log::info!("Database connection established.");

        Ok(Self { pool })
    }

    async fn create_database(url: &str) -> Result<(), DbError> {
        log::info!("Database not exists.");
        Postgres::create_database(url)
            .await
            .map_err(DbError::Creation)?;
        log::info!("Database successfully created.");

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Failed to connect to the database. {0}")]
    Connection(sqlx::Error),

    #[error("Failed to create database. {0}")]
    Creation(sqlx::Error),

    #[error("Failed to check is database exists or not. {0}")]
    ExistsOrNotValidation(sqlx::Error),
}
