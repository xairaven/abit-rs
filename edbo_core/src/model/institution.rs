use crate::dto::institution::InstitutionDto;
use crate::model::ModelError;
use crate::model::region::Region;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug)]
pub struct Institution {
    pub name: String,
    pub id: u16,
    pub parent_id: Option<u16>,
    pub short_name: Option<String>,
    pub english_name: Option<String>,
    pub is_from_crimea: bool,
    pub registration_year: Option<u16>,
    pub category: InstitutionCategory,
    pub ownership_form: OwnershipForm,
    pub region: Region,
}

impl TryFrom<InstitutionDto> for Institution {
    type Error = ModelError;

    fn try_from(value: InstitutionDto) -> Result<Self, Self::Error> {
        let parent_id = if let Some(parent_id) = value.university_parent_id {
            Some(parent_id.parse::<u16>().map_err(|err| {
                ModelError::Institution(InstitutionError::FailedParseParentId(err))
            })?)
        } else {
            None
        };

        let english_name = if let Some(value) = &value.university_name_en
            && value.trim().is_empty()
        {
            None
        } else {
            value.university_name_en
        };

        let is_from_crimea = matches!(value.is_from_crimea.as_str(), "так");

        let registration_year = if let Some(year) = value.registration_year {
            Some(year.parse::<u16>().map_err(|err| {
                ModelError::Institution(InstitutionError::FailedParseRegistrationYear(
                    err,
                ))
            })?)
        } else {
            None
        };

        Ok(Self {
            name: value.university_name,
            id: value.university_id.parse().map_err(|err| {
                ModelError::Institution(InstitutionError::FailedParseId(err))
            })?,
            parent_id,
            short_name: value.university_short_name,
            english_name,
            is_from_crimea,
            registration_year,
            category: InstitutionCategory::from(
                value.university_type_name.unwrap_or_default().as_str(),
            ),
            ownership_form: OwnershipForm::from(
                value
                    .university_financing_type_name
                    .unwrap_or_default()
                    .as_str(),
            ),
            region: Region::try_from(value.region_name_u.as_str())
                .map_err(ModelError::Region)?,
        })
    }
}

#[derive(Debug)]
pub enum InstitutionCategory {
    // UA: Заклади вищої освіти
    HigherEducation = 1,

    // UA: Заклади професійної (професійно-технічної) освіти
    VocationalTechnical = 2,

    // UA: Заклади фахової передвищої освіти
    PreUniversityProfessional = 9,

    // UA: Наукові інститути (установи)
    Scientific = 8,

    // UA: Заклади післядипломної освіти
    Postgraduate = 10,

    // UA: Заклад загальної середньої освіти
    GeneralSecondaryEducation,

    // UA: Інший заклад освіти, що надає професійну (професійно-технічну освіту)
    OtherVocationalTechnical,

    Unknown,
}

impl From<&str> for InstitutionCategory {
    fn from(value: &str) -> Self {
        match value {
            "Заклад вищої освіти" => Self::HigherEducation,
            "Заклад загальної середньої освіти" => {
                Self::GeneralSecondaryEducation
            },
            "Заклад післядипломної освіти" => {
                Self::Postgraduate
            },
            "Заклад професійної (професійно-технічної) освіти" => {
                Self::VocationalTechnical
            },
            "Заклад фахової передвищої освіти" => {
                Self::PreUniversityProfessional
            },
            "Інший заклад освіти, що надає професійну (професійно-технічну освіту)" => {
                Self::OtherVocationalTechnical
            },
            "Наукові інститути (установи)" => Self::Scientific,
            _ => Self::Unknown,
        }
    }
}

impl InstitutionCategory {
    pub const fn code(&self) -> Option<u16> {
        match self {
            Self::HigherEducation => Some(1),
            Self::VocationalTechnical => Some(2),
            Self::PreUniversityProfessional => Some(9),
            Self::Scientific => Some(8),
            Self::Postgraduate => Some(10),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum OwnershipForm {
    State,
    Municipal,
    Corporate,
    Private,
    Unknown,
}

impl From<&str> for OwnershipForm {
    fn from(value: &str) -> Self {
        match value {
            "Державна" => Self::State,
            "Комунальна" => Self::Municipal,
            "Корпоративна" => Self::Corporate,
            "Приватна" => Self::Private,
            _ => Self::Unknown,
        }
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
}
