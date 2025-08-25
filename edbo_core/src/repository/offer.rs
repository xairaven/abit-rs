use crate::database::Database;
use crate::model::ModelError;
use crate::model::degree::{Degree, DegreeError};
use crate::model::offer::Offer;
use crate::model::speciality::{Speciality, SpecialityError};
use crate::model::study_form::{StudyForm, StudyFormError};
use crate::repository::{Repository, RepositoryError, RepositoryResult};

pub struct OfferRepository<'a> {
    db: &'a Database,
}

#[async_trait::async_trait]
impl<'a> Repository<'a> for OfferRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }

    async fn is_empty(&self) -> RepositoryResult<bool> {
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM offer);")
            .fetch_one(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(!result.exists.unwrap_or(false))
    }
}

impl<'a> OfferRepository<'a> {
    pub async fn create(&self, offer: &Offer) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO offer (id, title, degree_id, education_program, study_form_id, faculty, speciality_code, master_type, license_volume, budgetary_places)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            offer.id,
            offer.title,
            (offer.degree as i16),
            offer.education_program,
            (offer.study_form as i16),
            offer.faculty,
            offer.speciality.code(),
            offer.master_type,
            offer.license_volume,
            offer.budgetary_places
        )
        .execute(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: i32) -> RepositoryResult<Vec<Offer>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, title, degree_id, education_program, study_form_id, faculty, speciality_code, master_type, license_volume, budgetary_places
            FROM offer
            WHERE id = $1
        "#,
            id
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        let mut offers = Vec::new();
        for row in rows {
            let offer = Offer {
                id: row.id,
                title: row.title,
                degree: Degree::try_from(row.degree_id as i8)
                    .map_err(DegreeError::UnknownDegree)
                    .map_err(ModelError::Degree)?,
                education_program: row.education_program,
                faculty: row.faculty,
                speciality: Speciality::try_from(row.speciality_code.as_str())
                    .map_err(ModelError::Speciality)?,
                master_type: row.master_type,
                license_volume: row.license_volume,
                study_form: StudyForm::try_from(row.study_form_id as i8)
                    .map_err(|_| StudyFormError::UnknownId(row.study_form_id as i8))
                    .map_err(ModelError::StudyForm)?,
                budgetary_places: row.budgetary_places,
            };
            offers.push(offer);
        }

        Ok(offers)
    }

    pub async fn find_all(&self) -> RepositoryResult<Vec<Offer>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, title, degree_id, education_program, study_form_id, faculty, speciality_code, master_type, license_volume, budgetary_places
            FROM offer
        "#)
            .fetch_all(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        let mut offers = Vec::new();
        for row in rows {
            let offer = Offer {
                id: row.id,
                title: row.title,
                degree: Degree::try_from(row.degree_id as i8)
                    .map_err(DegreeError::UnknownDegree)
                    .map_err(ModelError::Degree)?,
                education_program: row.education_program,
                study_form: StudyForm::try_from(row.study_form_id as i8)
                    .map_err(|_| StudyFormError::UnknownId(row.study_form_id as i8))
                    .map_err(ModelError::StudyForm)?,
                faculty: row.faculty,
                speciality: Speciality::try_from(row.speciality_code.as_str())
                    .map_err(|_| {
                        SpecialityError::UnknownSpecialityCode(row.speciality_code)
                    })
                    .map_err(ModelError::Speciality)?,
                master_type: row.master_type,
                license_volume: row.license_volume,
                budgetary_places: row.budgetary_places,
            };
            offers.push(offer);
        }

        Ok(offers)
    }

    pub async fn update(&self, offer: &Offer) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
                UPDATE offer
                SET title = $1,
                    degree_id = $2,
                    education_program = $3,
                    faculty = $4,
                    speciality_code = $5,
                    master_type = $6,
                    study_form_id = $7,
                    license_volume = $8,
                    budgetary_places = $9
                WHERE id = $10
            "#,
            offer.title,
            (offer.degree as i16),
            offer.education_program,
            offer.faculty,
            offer.speciality.code(),
            offer.master_type,
            (offer.study_form as i16),
            offer.license_volume,
            offer.budgetary_places,
            offer.id
        )
        .execute(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        Ok(result.rows_affected())
    }

    pub async fn delete(&self, id: i32) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
                DELETE FROM offer
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
