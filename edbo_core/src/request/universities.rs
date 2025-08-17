use crate::dto::universities::UniversityDto;
use crate::model::university::InstitutionCategory;
use crate::request::{ExportFormat, RequestError};
use url::Url;

pub async fn list() -> Result<(), RequestError> {
    const BASE_URL: &str = "https://registry.edbo.gov.ua/api/universities/";
    let mut url = Url::parse(BASE_URL).map_err(RequestError::FailedToParseUrl)?;

    const PARAMETERS: UniversitiesApi = UniversitiesApi {
        category: Some(InstitutionCategory::HigherEducation as u8),
        region_code: None,
        export_format: Some(ExportFormat::Json.into_static_str()),
    };
    PARAMETERS.url_append_parameters(&mut url);

    let client = reqwest::Client::builder()
        .build()
        .map_err(RequestError::FailedBuildClient)?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(RequestError::RequestFailed)?;

    let list: Vec<UniversityDto> = response
        .json()
        .await
        .map_err(RequestError::JsonParseFailed)?;

    dbg!(list);

    Ok(())
}

pub struct UniversitiesApi {
    pub category: Option<u8>,
    pub region_code: Option<u8>,
    pub export_format: Option<&'static str>,
}

impl UniversitiesApi {
    pub fn url_append_parameters(&self, url: &mut Url) {
        const CATEGORY_KEY: &str = "ut";
        const REGION_CODE_KEY: &str = "lc";
        const EXPORT_FORMAT_KEY: &str = "exp";

        if let Some(category_value) = self.category {
            url.query_pairs_mut()
                .append_pair(CATEGORY_KEY, &category_value.to_string());
        }
        if let Some(region_code) = self.region_code {
            url.query_pairs_mut()
                .append_pair(REGION_CODE_KEY, &region_code.to_string());
        }
        if let Some(format) = self.export_format {
            url.query_pairs_mut().append_pair(EXPORT_FORMAT_KEY, format);
        }
    }
}
