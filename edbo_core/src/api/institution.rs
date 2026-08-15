use crate::api::{ApiError, ApiFetcherUrl, ExportFormat};
use crate::dto::institution::InstitutionDto;
use crate::error::CoreError;
use crate::model::institution::Institution;
use crate::{api, request};
use url::Url;

pub async fn list() -> Result<Vec<Institution>, CoreError> {
    let base_url = format!("{}/universities/", api::links::REGISTRY);
    let mut url = Url::parse(&base_url).map_err(ApiError::FailedToParseUrl)?;

    const PARAMETERS: InstitutionsApi = InstitutionsApi {
        category: None,
        region_code: None,
        export_format: Some(ExportFormat::Json.into_static_str()),
    };
    PARAMETERS.append_parameters_to_url(&mut url);

    let client = request::Client::build()?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(ApiError::RequestFailed)?;
    log::info!("Institution list response success.");

    let text = response
        .text()
        .await
        .map_err(ApiError::FailedToGetResponseText)?;
    log::debug!("Text from response: {:?}", text);
    let dto_list: Vec<InstitutionDto> =
        serde_json::from_str(&text).map_err(ApiError::JsonParseFailed)?;

    let mut list: Vec<Institution> = Vec::with_capacity(dto_list.len());
    for dto in dto_list {
        let value = Institution::try_from(dto)?;
        list.push(value);
    }

    Ok(list)
}

pub struct InstitutionsApi {
    pub category: Option<u16>,
    pub region_code: Option<u8>,
    pub export_format: Option<&'static str>,
}

impl ApiFetcherUrl for InstitutionsApi {
    fn append_parameters_to_url(&self, url: &mut Url) {
        const CATEGORY_KEY: &str = "ut";
        const REGION_CODE_KEY: &str = "lc";
        const EXPORT_FORMAT_KEY: &str = "exp";

        Self::append_optional_parameter(url, CATEGORY_KEY, &self.category);
        Self::append_optional_parameter(url, REGION_CODE_KEY, &self.region_code);
        Self::append_optional_parameter(url, EXPORT_FORMAT_KEY, &self.export_format);
    }
}
