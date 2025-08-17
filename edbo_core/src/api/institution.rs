use crate::api::{ApiError, ExportFormat};
use crate::dto::institution::InstitutionDto;
use crate::error::CoreError;
use crate::model::institution::Institution;
use url::Url;

pub async fn list() -> Result<Vec<Institution>, CoreError> {
    type Error = ApiError;

    const BASE_URL: &str = "https://registry.edbo.gov.ua/api/universities/";
    let mut url = Url::parse(BASE_URL).map_err(Error::FailedToParseUrl)?;

    const PARAMETERS: InstitutionsApi = InstitutionsApi {
        category: None,
        region_code: None,
        export_format: Some(ExportFormat::Json.into_static_str()),
    };
    PARAMETERS.url_append_parameters(&mut url);

    let client = reqwest::Client::builder()
        .build()
        .map_err(Error::FailedBuildClient)?;

    let response = client.get(url).send().await.map_err(Error::RequestFailed)?;

    let dto_list: Vec<InstitutionDto> =
        response.json().await.map_err(Error::JsonParseFailed)?;

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

impl InstitutionsApi {
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
