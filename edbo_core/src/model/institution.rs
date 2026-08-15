use crate::dto::institution::InstitutionDto;
use crate::model::ModelError;
use crate::model::institution_category::InstitutionCategory;
use crate::model::ownership_form::OwnershipForm;
use crate::model::region::Region;
use num_enum::TryFromPrimitiveError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug)]
pub struct Institution {
    pub name: String,
    pub id: i16,
    pub parent_id: Option<i16>,
    pub short_name: Option<String>,
    pub english_name: Option<String>,
    pub is_from_crimea: bool,
    pub registration_year: Option<i16>,
    pub category: InstitutionCategory,
    pub ownership_form: OwnershipForm,
    pub region: Region,
}

impl TryFrom<InstitutionDto> for Institution {
    type Error = ModelError;

    fn try_from(dto: InstitutionDto) -> Result<Self, Self::Error> {
        let parent_id = if let Some(parent_id) = dto.university_parent_id {
            Some(parent_id.parse::<i16>().map_err(|err| {
                ModelError::Institution(InstitutionError::FailedParseParentId(err))
            })?)
        } else {
            None
        };

        let english_name = if let Some(value) = &dto.university_name_en
            && value.trim().is_empty()
        {
            None
        } else {
            dto.university_name_en
        };

        let is_from_crimea = matches!(dto.is_from_crimea.as_str(), "так");

        let registration_year = if let Some(year) = dto.registration_year {
            Some(year.parse::<i16>().map_err(|err| {
                ModelError::Institution(InstitutionError::FailedParseRegistrationYear(
                    err,
                ))
            })?)
        } else {
            None
        };

        Ok(Self {
            name: dto.university_name,
            id: dto.university_id.parse().map_err(|err| {
                ModelError::Institution(InstitutionError::FailedParseId(err))
            })?,
            parent_id,
            short_name: dto.university_short_name,
            english_name,
            is_from_crimea,
            registration_year,
            category: InstitutionCategory::from(
                dto.university_type_name.unwrap_or_default().as_str(),
            ),
            ownership_form: OwnershipForm::from(
                dto.university_financing_type_name
                    .unwrap_or_default()
                    .as_str(),
            ),
            region: Region::try_from(dto.region_name_u.as_str())
                .map_err(ModelError::Region)?,
        })
    }
}

#[derive(Debug, Error)]
pub enum InstitutionError {
    #[error("Failed to parse institution id. {0}")]
    FailedParseId(ParseIntError),

    #[error("Failed to parse institution parent id. {0}")]
    FailedParseParentId(ParseIntError),

    #[error("Failed to parse registration year. {0}")]
    FailedParseRegistrationYear(ParseIntError),

    #[error("Failed to parse institution category id. {0}")]
    FailedParseCategoryId(TryFromPrimitiveError<InstitutionCategory>),

    #[error("Failed to parse institution ownership form id. {0}")]
    FailedParseOwnershipFormId(TryFromPrimitiveError<OwnershipForm>),

    #[error("Failed to parse institution region id. {0}")]
    FailedParseRegionId(TryFromPrimitiveError<Region>),
}
