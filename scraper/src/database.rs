use model::schemas;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Debug)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn configure(&self) -> Result<(), DbError> {
        let is_migration_needed = self.is_migration_needed().await?;
        if is_migration_needed {
            log::info!("There are no tables. Need to do migration...");
            sqlx::migrate!().run(&self.pool).await?;
            log::info!("Migration done successfully.");
        }

        Ok(())
    }

    async fn is_migration_needed(&self) -> Result<bool, DbError> {
        let exists_schema_common: bool = sqlx::query_scalar(
            "SELECT EXISTS ( SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)"
        )
            .bind(schemas::COMMON)
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::SchemaExistsValidation)?;

        let exists_schema_scraped: bool = sqlx::query_scalar(
            "SELECT EXISTS ( SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)"
        )
            .bind(schemas::SCRAPED)
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::SchemaExistsValidation)?;

        // Check if the common and scraped schema contains any user-defined base tables
        let has_tables_common: bool = sqlx::query_scalar(
            "SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = $1
            AND table_type = 'BASE TABLE'
        )",
        )
        .bind(schemas::COMMON)
        .fetch_one(&self.pool)
        .await
        .map_err(DbError::TableAmountValidation)?;

        let has_tables_scraped: bool = sqlx::query_scalar(
            "SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = $1
            AND table_type = 'BASE TABLE'
        )",
        )
        .bind(schemas::SCRAPED)
        .fetch_one(&self.pool)
        .await
        .map_err(DbError::TableAmountValidation)?;

        let result = exists_schema_common
            && exists_schema_scraped
            && has_tables_scraped
            && has_tables_common;

        Ok(result)
    }
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Failed to run database migrations. {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Failed to execute query that checks if schema exists. {0}")]
    SchemaExistsValidation(sqlx::Error),

    #[error("Failed to execute query that checks amount of tables. {0}")]
    TableAmountValidation(sqlx::Error),
}
