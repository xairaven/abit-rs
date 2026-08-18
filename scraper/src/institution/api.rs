use crate::institution::dto::InstitutionDto;
use crate::institution::errors::InstitutionError;
use model::institution::category::InstitutionCategory;
use model::region::Region;

pub struct InstitutionApi;

impl InstitutionApi {
    pub async fn list() -> Result<Vec<InstitutionDto>, InstitutionError> {
        let url = Self::build_url();
        let text = reqwest::get(url)
            .await
            .map_err(InstitutionError::Request)?
            .text()
            .await
            .map_err(InstitutionError::RequestText)?;
        let institutions =
            serde_json::from_str(&text).map_err(InstitutionError::Deserializing)?;

        Ok(institutions)
    }

    // `rg` = region filter (0 = every region, see model::region::Region::Every),
    // `ut` = institution category filter (see model::institution::category::InstitutionCategory).
    // Only category 1 (institutions of higher education) offers master's programs,
    // so that's the only category this project needs — not looping over the rest.
    fn build_url() -> String {
        format!(
            "https://registry.edbo.gov.ua/api/opendata/universities?rg={}&ut={}&exp=json",
            i16::from(Region::Every),
            i16::from(InstitutionCategory::HigherEducation)
        )
    }
}
