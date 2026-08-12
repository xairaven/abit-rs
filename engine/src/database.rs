use crate::EngineConfig;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres};
use thiserror::Error;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn init(config: &EngineConfig) -> Result<Self, DbError> {
        let url = &config.database_url;

        let is_db_exists_at_initialization = Postgres::database_exists(url)
            .await
            .map_err(DbError::IsPresentCheck)?;

        if is_db_exists_at_initialization {
            log::info!("Engine database exists.");
        } else {
            Self::create_database(url).await?;
        }

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .map_err(DbError::Connection)?;

        log::info!("Database connection established.");

        let is_migration_needed = Self::is_migration_needed(&pool).await?;
        if is_migration_needed {
            log::info!("There are no tables. Need to do migration...");
            sqlx::migrate!().run(&pool).await?;
            log::info!("Migration done successfully.");
        }

        Ok(Self { pool })
    }

    async fn create_database(url: &str) -> Result<(), DbError> {
        log::info!("Engine database not exists.");
        Postgres::create_database(url)
            .await
            .map_err(DbError::CoreCreation)?;
        log::info!("Engine database successfully created.");

        Ok(())
    }

    async fn is_migration_needed(pool: &PgPool) -> Result<bool, DbError> {
        // Check if the public schema contains any user-defined base tables
        let has_tables: bool = sqlx::query_scalar(
            "SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_type = 'BASE TABLE'
        )",
        )
        .fetch_one(pool)
        .await
        .map_err(DbError::TableAmountCheck)?;

        Ok(has_tables)
    }
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Failed to connect to the engine database. {0}")]
    Connection(sqlx::Error),

    #[error("Failed to create engine database. {0}")]
    CoreCreation(sqlx::Error),

    #[error("Failed to check is database exists or not. {0}")]
    IsPresentCheck(sqlx::Error),

    #[error("Failed to run database migrations. {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Failed to execute query that checks amount of tables. {0}")]
    TableAmountCheck(sqlx::Error),
}
