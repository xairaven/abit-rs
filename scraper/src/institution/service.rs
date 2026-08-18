use crate::database::Database;
use crate::institution::api::InstitutionApi;
use crate::institution::errors::InstitutionError;
use model::institution::Institution;
use model::institution::category::InstitutionCategory;
use model::institution::ownership::OwnershipForm;
use model::region::Region;

pub struct InstitutionService<'a> {
    database: &'a Database,
}

impl<'a> InstitutionService<'a> {
    pub const fn new(database: &'a Database) -> Self {
        Self { database }
    }

    pub async fn get(&self) -> Result<Vec<Institution>, InstitutionError> {
        if self.is_empty().await? {
            log::info!("Institutions table is empty, fetching from EDBO...");
            let dtos = InstitutionApi::list().await?;

            let mut institutions = Vec::with_capacity(dtos.len());
            for dto in dtos {
                institutions.push(Institution::try_from(dto)?);
            }

            for institution in &institutions {
                self.insert(institution).await?;
            }

            log::info!("Inserted {} institutions.", institutions.len());
            Ok(institutions)
        } else {
            log::info!("Institutions table is already populated, reading from DB...");
            self.find_all().await
        }
    }

    async fn is_empty(&self) -> Result<bool, InstitutionError> {
        let empty =
            sqlx::query_scalar!("SELECT NOT EXISTS (SELECT 1 FROM common.institution)")
                .fetch_one(self.database.pool())
                .await
                .map_err(InstitutionError::IsEmpty)?;

        Ok(empty.unwrap_or(true))
    }

    async fn insert(&self, institution: &Institution) -> Result<(), InstitutionError> {
        sqlx::query!(
            "INSERT INTO common.institution
            (id, name, parent_id, short_name, english_name, is_from_crimea,
             registration_date, category_id, ownership_form_id, region_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            institution.id,
            institution.title,
            institution.parent_id,
            institution.short_name,
            institution.english_name,
            institution.is_from_crimea,
            institution.registration_date,
            i16::from(institution.category),
            i16::from(institution.ownership_form),
            institution.region.map(i16::from),
        )
        .execute(self.database.pool())
        .await
        .map_err(InstitutionError::Insert)?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Institution>, InstitutionError> {
        let rows = sqlx::query!(
            "SELECT id, name, parent_id, short_name, english_name, is_from_crimea,
                registration_date, category_id, ownership_form_id, region_id
         FROM common.institution"
        )
        .fetch_all(self.database.pool())
        .await
        .map_err(InstitutionError::FindAll)?;

        let mut institutions = Vec::with_capacity(rows.len());

        for row in rows {
            let category =
                InstitutionCategory::try_from(row.category_id).map_err(|err| {
                    InstitutionError::InconsistentCategoryData(err.to_string())
                })?;
            let ownership_form =
                OwnershipForm::try_from(row.ownership_form_id).map_err(|err| {
                    InstitutionError::InconsistentOwnershipFormData(err.to_string())
                })?;
            let region =
                row.region_id
                    .map(Region::try_from)
                    .transpose()
                    .map_err(|err| {
                        InstitutionError::InconsistentRegionData(err.to_string())
                    })?;

            institutions.push(Institution {
                title: row.name,
                id: row.id,
                parent_id: row.parent_id,
                short_name: row.short_name,
                english_name: row.english_name,
                is_from_crimea: row.is_from_crimea,
                registration_date: row.registration_date,
                category,
                ownership_form,
                region,
            });
        }

        Ok(institutions)
    }
}
