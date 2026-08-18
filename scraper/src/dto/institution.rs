use crate::dto::DtoError;
use model::institution::Institution;
use model::institution::category::InstitutionCategory;
use model::institution::ownership::OwnershipForm;
use model::region::Region;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct InstitutionDto {
    #[serde(rename = "Назва закладу освіти")]
    pub title: String,
    #[serde(rename = "Код")]
    pub id: i16,
    #[serde(rename = "Код головного закладу")]
    pub parent_id: Option<i16>,
    #[serde(rename = "Коротка назва")]
    pub short_name: String,
    #[serde(rename = "Назва закладу освіти (англ.)")]
    pub english_name: String,
    #[serde(rename = "ОЦ «Крим-Україна», ОЦ «Донбас-Україна»")]
    pub is_from_crimea: String,
    #[serde(rename = "Рік заснування")]
    pub registration_date: String,
    #[serde(rename = "Категорія закладу освіти")]
    pub category: String,
    #[serde(rename = "Форма власності")]
    pub ownership_form: String,
    #[serde(rename = "Регіон (місцезнаходження)")]
    pub region: Option<String>,
}

impl TryFrom<InstitutionDto> for Institution {
    type Error = DtoError;

    fn try_from(value: InstitutionDto) -> Result<Self, Self::Error> {
        let short_name = if value.short_name.is_empty() {
            None
        } else {
            Some(value.short_name)
        };

        let english_name = if value.english_name.is_empty() {
            None
        } else {
            Some(value.english_name)
        };

        let is_from_crimea = value.is_from_crimea.eq("Так");

        let registration_date = if value.registration_date.is_empty() {
            None
        } else {
            Some(value.registration_date)
        };

        let category = InstitutionCategory::from_str(&value.category)
            .map_err(Self::Error::InstitutionCategory)?;

        let ownership_form = OwnershipForm::from_str(&value.ownership_form)
            .map_err(Self::Error::OwnershipForm)?;

        let region = value
            .region
            .filter(|region| !region.is_empty())
            .map(|region| Region::from_str(&region))
            .transpose()
            .map_err(Self::Error::Region)?;

        let institution = Self {
            title: value.title,
            id: value.id,
            parent_id: value.parent_id,
            short_name,
            english_name,
            is_from_crimea,
            registration_date,
            category,
            ownership_form,
            region,
        };

        Ok(institution)
    }
}
