use crate::database::Database;
use crate::model::ModelError;
use crate::model::institution::{Institution, InstitutionError};
use crate::model::institution_category::InstitutionCategory;
use crate::model::ownership_form::OwnershipForm;
use crate::model::region::Region;
use crate::repository::{Repository, RepositoryError, RepositoryResult};

pub struct InstitutionRepository<'a> {
    db: &'a Database,
}

#[async_trait::async_trait]
impl<'a> Repository<'a> for InstitutionRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }

    async fn is_empty(&self) -> RepositoryResult<bool> {
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM institution);")
            .fetch_one(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(!result.exists.unwrap_or(false))
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
            (institution.id as i32),
            institution.name,
            institution.parent_id.map(|id| id as i32),
            institution.short_name,
            institution.english_name,
            institution.is_from_crimea,
            institution.registration_year,
            (institution.category as i16),
            (institution.ownership_form as i16),
            (institution.region as i16)
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
                category: InstitutionCategory::try_from(row.category_id as i8)
                    .map_err(InstitutionError::FailedParseCategoryId)
                    .map_err(ModelError::Institution)?,
                ownership_form: OwnershipForm::try_from(row.ownership_form_id as i8)
                    .map_err(InstitutionError::FailedParseOwnershipFormId)
                    .map_err(ModelError::Institution)?,
                region: Region::try_from(row.region_id as i8)
                    .map_err(InstitutionError::FailedParseRegionId)
                    .map_err(ModelError::Institution)?,
            };
            institutions.push(institution);
        }

        Ok(institutions)
    }

    pub async fn find_all(&self) -> RepositoryResult<Vec<Institution>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, parent_id, short_name, english_name, is_from_crimea,
                   registration_year, category_id, ownership_form_id, region_id FROM institution
            "#
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
                category: InstitutionCategory::try_from(row.category_id as i8)
                    .map_err(InstitutionError::FailedParseCategoryId)
                    .map_err(ModelError::Institution)?,
                ownership_form: OwnershipForm::try_from(row.ownership_form_id as i8)
                    .map_err(InstitutionError::FailedParseOwnershipFormId)
                    .map_err(ModelError::Institution)?,
                region: Region::try_from(row.region_id as i8)
                    .map_err(InstitutionError::FailedParseRegionId)
                    .map_err(ModelError::Institution)?,
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
