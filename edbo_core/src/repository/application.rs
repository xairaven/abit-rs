use crate::database::Database;
use crate::model::ModelError;
use crate::model::applicant::GradeComponentError;
use crate::model::application::Application;
use crate::model::priority::Priority;
use crate::model::status::{ApplicationStatus, ApplicationStatusError};
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use bigdecimal::BigDecimal;
use num_traits::cast::ToPrimitive;

pub struct ApplicationRepository<'a> {
    db: &'a Database,
}

#[async_trait::async_trait]
impl<'a> Repository<'a> for ApplicationRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }

    async fn is_empty(&self) -> RepositoryResult<bool> {
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM application);")
            .fetch_one(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(!result.exists.unwrap_or(false))
    }
}

impl<'a> ApplicationRepository<'a> {
    pub async fn create(&self, application: &Application) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO application (number_in_list, status_id, grade, priority_code, offer_id, user_id)
                VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            application.number_in_list,
            Into::<i8>::into(application.status) as i16,
            BigDecimal::try_from(application.grade)
                .map_err(|err| ModelError::GradeComponent(GradeComponentError::FailedToBigDecimal(err)))?,
            Into::<i8>::into(application.priority) as i16,
            application.offer_id,
            application.user_id
        )
        .execute(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        Ok(())
    }

    pub async fn find_by_offer_id(&self, id: i32) -> RepositoryResult<Vec<Application>> {
        let rows = sqlx::query!(
            r#"
            SELECT number_in_list, status_id, grade, priority_code, offer_id, user_id
            FROM application
            WHERE offer_id = $1
        "#,
            id
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        let mut applications = Vec::new();
        for row in rows {
            let application = Application {
                number_in_list: row.number_in_list,
                status: ApplicationStatus::try_from(row.status_id as i8).map_err(
                    |_| {
                        ModelError::ApplicationStatus(
                            ApplicationStatusError::UnknownCode(row.status_id as i32),
                        )
                    },
                )?,
                grade: row.grade.to_f32().ok_or(ModelError::GradeComponent(
                    GradeComponentError::FailedFromBigInt,
                ))?,
                priority: Priority::from(row.priority_code as i8),
                offer_id: row.offer_id,
                user_id: row.user_id,
            };
            applications.push(application);
        }

        Ok(applications)
    }

    pub async fn find_by_user_id(&self, id: i32) -> RepositoryResult<Vec<Application>> {
        let rows = sqlx::query!(
            r#"
            SELECT number_in_list, status_id, grade, priority_code, offer_id, user_id
            FROM application
            WHERE user_id = $1
        "#,
            id
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        let mut applications = Vec::new();
        for row in rows {
            let application = Application {
                number_in_list: row.number_in_list,
                status: ApplicationStatus::try_from(row.status_id as i8).map_err(
                    |_| {
                        ModelError::ApplicationStatus(
                            ApplicationStatusError::UnknownCode(row.status_id as i32),
                        )
                    },
                )?,
                grade: row.grade.to_f32().ok_or(ModelError::GradeComponent(
                    GradeComponentError::FailedFromBigInt,
                ))?,
                priority: Priority::from(row.priority_code as i8),
                offer_id: row.offer_id,
                user_id: row.user_id,
            };
            applications.push(application);
        }

        Ok(applications)
    }

    pub async fn find_all(&self) -> RepositoryResult<Vec<Application>> {
        let rows = sqlx::query!(
            r#"
            SELECT number_in_list, status_id, grade,
                   priority_code, offer_id, user_id FROM application
                   "#
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        let mut applications = Vec::new();
        for row in rows {
            let application = Application {
                number_in_list: row.number_in_list,
                status: ApplicationStatus::try_from(row.status_id as i8).map_err(
                    |_| {
                        ModelError::ApplicationStatus(
                            ApplicationStatusError::UnknownCode(row.status_id as i32),
                        )
                    },
                )?,
                grade: row.grade.to_f32().ok_or(ModelError::GradeComponent(
                    GradeComponentError::FailedFromBigInt,
                ))?,
                priority: Priority::from(row.priority_code as i8),
                offer_id: row.offer_id,
                user_id: row.user_id,
            };
            applications.push(application);
        }

        Ok(applications)
    }
}
