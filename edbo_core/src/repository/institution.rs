use crate::database::Database;
use crate::model::ModelError;
use crate::model::institution::{Institution, InstitutionError};
use crate::model::institution_category::InstitutionCategory;
use crate::model::ownership_form::OwnershipForm;
use crate::model::region::Region;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use sqlx::Row;

pub struct InstitutionRepository<'a> {
    db: &'a Database,
}

impl<'a> Repository<'a> for InstitutionRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }
}

impl<'a> InstitutionRepository<'a> {
    pub async fn create(&self, institution: &Institution) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO institution (id, name, parent_id, short_name, english_name,
                                         is_from_crimea, registration_year, category_id, ownership_form_id, region_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            institution.id as i32,
            institution.name,
            institution.parent_id.map(|id| id as i32),
            institution.short_name,
            institution.english_name,
            institution.is_from_crimea,
            institution.registration_year,
            institution.category as i8,
            institution.ownership_form as i8,
            institution.region as i8
        )
        .execute(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: i32) -> RepositoryResult<Vec<Institution>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, parent_id, short_name, english_name,
                                         is_from_crimea, registration_year, category_id, ownership_form_id, region_id
            FROM institution
            WHERE id = $1
        "#,
            id
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        let mut institutions = Vec::new();
        for row in rows {
            let institution = Institution {
                name: row.name,
                id: row.id as i16,
                parent_id: row.parent_id.map(|id| id as i16),
                short_name: row.short_name,
                english_name: row.english_name,
                is_from_crimea: row.is_from_crimea,
                registration_year: row.registration_year,
                category: InstitutionCategory::try_from(row.category_id as i8).map_err(
                    |err| {
                        ModelError::Institution(InstitutionError::FailedParseCategoryId(
                            err,
                        ))
                    },
                )?,
                ownership_form: OwnershipForm::try_from(row.ownership_form_id as i8)
                    .map_err(|err| {
                        ModelError::Institution(
                            InstitutionError::FailedParseOwnershipFormId(err),
                        )
                    })?,
                region: Region::try_from(row.region_id as i8).map_err(|err| {
                    ModelError::Institution(InstitutionError::FailedParseRegionId(err))
                })?,
            };
            institutions.push(institution);
        }

        Ok(institutions)
    }

    pub async fn find_all(
        &self, limit: Option<i32>, offset: Option<i32>,
    ) -> RepositoryResult<Vec<Institution>> {
        let select = "SELECT id, name, parent_id, short_name, english_name, is_from_crimea, registration_year, category_id, ownership_form_id, region_id FROM application";

        let query = match (limit, offset) {
            (Some(l), Some(o)) => format!("{} LIMIT {} OFFSET {}", select, l, o),
            (Some(l), None) => format!("{} LIMIT {}", select, l),
            (None, Some(o)) => format!("{} OFFSET {}", select, o),
            (None, None) => select.to_string(),
        };

        let rows = sqlx::query(&query)
            .fetch_all(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        let mut institutions = Vec::new();
        for row in rows {
            let institution = Institution {
                name: row.get(0),
                id: row.get::<i32, _>(1) as i16,
                parent_id: row.get::<Option<i32>, _>(2).map(|value| value as i16),
                short_name: row.get(3),
                english_name: row.get(4),
                is_from_crimea: row.get(5),
                registration_year: row.get::<Option<i32>, _>(6).map(|value| value as i16),
                category: InstitutionCategory::try_from(row.get::<i16, _>(7) as i8)
                    .map_err(|err| {
                        ModelError::Institution(InstitutionError::FailedParseCategoryId(
                            err,
                        ))
                    })?,
                ownership_form: OwnershipForm::try_from(row.get::<i16, _>(8) as i8)
                    .map_err(|err| {
                        ModelError::Institution(
                            InstitutionError::FailedParseOwnershipFormId(err),
                        )
                    })?,
                region: Region::try_from(row.get::<i16, _>(9) as i8).map_err(|err| {
                    ModelError::Institution(InstitutionError::FailedParseRegionId(err))
                })?,
            };
            institutions.push(institution);
        }

        Ok(institutions)
    }

    pub async fn update(&self, institution: &Institution) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
                UPDATE institution
                SET name = $1, parent_id = $2, short_name = $3, english_name = $4,
                    is_from_crimea = $5, registration_year = $6, category_id = $7, ownership_form_id = $8, region_id = $9
                WHERE id = $10
            "#,
            institution.name,
            institution.parent_id.map(|id| id as i32),
            institution.short_name,
            institution.english_name,
            institution.is_from_crimea,
            institution.registration_year,
            institution.category as i8,
            institution.ownership_form as i8,
            institution.region as i8,
            institution.id as i32
        )
        .execute(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        Ok(result.rows_affected())
    }

    pub async fn delete(&self, id: i32) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
                DELETE FROM institution
                WHERE id = $1
            "#,
            id
        )
        .execute(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        Ok(result.rows_affected())
    }
}
