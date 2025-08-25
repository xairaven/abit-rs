use crate::database::Database;
use crate::model::applicant::{Applicant, GradeComponent};
use crate::repository::{Repository, RepositoryError, RepositoryResult};

pub struct ApplicantRepository<'a> {
    db: &'a Database,
}

#[async_trait::async_trait]
impl<'a> Repository<'a> for ApplicantRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }

    async fn is_empty(&self) -> RepositoryResult<bool> {
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM applicant);")
            .fetch_one(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(!result.exists.unwrap_or(false))
    }
}

impl<'a> ApplicantRepository<'a> {
    pub async fn create(&self, applicant: &Applicant) -> RepositoryResult<()> {
        let grade_components_json = serde_json::to_value(&applicant.grade_components)
            .map_err(RepositoryError::Json)?;

        sqlx::query!(
            r#"
                INSERT INTO applicant (id, name, grade_components)
                VALUES ($1, $2, $3)
            "#,
            applicant.id,
            applicant.name,
            grade_components_json
        )
        .execute(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Applicant>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, grade_components
            FROM applicant
            WHERE id = $1
        "#,
            id
        )
        .fetch_optional(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        match row {
            None => Ok(None),
            Some(row) => {
                let grade_components: Vec<GradeComponent> =
                    serde_json::from_value(row.grade_components)
                        .map_err(RepositoryError::Json)?;

                let applicant = Applicant {
                    id: row.id,
                    name: row.name,
                    grade_components,
                };

                Ok(Some(applicant))
            },
        }
    }

    pub async fn find_all(&self) -> RepositoryResult<Vec<Applicant>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, grade_components FROM applicant
        "#
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        let mut applicants = Vec::new();

        for row in rows {
            let id: i32 = row.id;
            let name: String = row.name;
            let grade_components_value: serde_json::Value = row.grade_components;

            let grade_components: Vec<GradeComponent> =
                serde_json::from_value(grade_components_value)
                    .map_err(RepositoryError::Json)?;

            applicants.push(Applicant {
                id,
                name,
                grade_components,
            });
        }

        Ok(applicants)
    }

    pub async fn update(&self, applicant: &Applicant) -> RepositoryResult<u64> {
        let grade_components_json = serde_json::to_value(&applicant.grade_components)
            .map_err(RepositoryError::Json)?;

        let result = sqlx::query!(
            r#"
                UPDATE applicant
                SET name = $1, grade_components = $2
                WHERE id = $3
            "#,
            applicant.name,
            grade_components_json,
            applicant.id
        )
        .execute(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        Ok(result.rows_affected())
    }

    pub async fn delete(&self, id: i32) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
                DELETE FROM applicant
                WHERE id = $1
            "#,
            id
        )
        .execute(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        Ok(result.rows_affected())
    }

    pub async fn truncate(&self) -> RepositoryResult<()> {
        sqlx::query!("TRUNCATE TABLE applicant;")
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(())
    }
}
