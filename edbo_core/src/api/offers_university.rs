use crate::api;
use crate::api::{
    ApiError, ApiFetcherForm, ApiFetcherUrl, ErrorResponse, INTERVAL_FOR_REQUESTS,
};
use crate::dto::offers_university::OffersUniversityMapDto;
use crate::error::CoreError;
use crate::model::degree::Degree;
use crate::model::offers_university::OffersUniversity;
use crate::model::{ModelError, speciality};
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use url::Url;

pub async fn list() -> Result<Vec<OffersUniversity>, CoreError> {
    let base_url = format!("{}/offers-universities/", api::links::MAIN);
    let base_url = Url::parse(&base_url).map_err(ApiError::FailedToParseUrl)?;

    let client = reqwest::Client::builder()
        .build()
        .map_err(ApiError::FailedBuildClient)?;

    let mut parameters = OffersUniversitiesApi {
        qualification: Some(Degree::Master.qualification().map_err(ModelError::Degree)?),
        education_base: Some(
            Degree::Bachelor
                .education_base()
                .map_err(ModelError::Degree)?,
        ),
        speciality: None,
        region: None,
        university: None,
        study_program: None,
        education_form: None,
        course: None,
    };

    let mut ticker = tokio::time::interval(INTERVAL_FOR_REQUESTS);

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static(api::USER_AGENT));
    headers.insert(
        "Referer",
        HeaderValue::from_str(base_url.as_str()).map_err(ApiError::InvalidHeaderValue)?,
    );

    let mut offers: Vec<OffersUniversity> = vec![];
    for (_, speciality) in speciality::ALL_SPECIALITIES.iter() {
        parameters.speciality = Some(speciality.code().to_string());

        ticker.tick().await;

        let form = parameters.create_form();

        let response = client
            .post(base_url.clone())
            .headers(headers.clone())
            .form(&form)
            .send()
            .await
            .map_err(ApiError::RequestFailed)?;

        let text = response
            .text()
            .await
            .map_err(ApiError::FailedToGetResponseText)?;
        log::debug!("Text from response: {:?}", text);

        let dto_map = loop {
            match serde_json::from_str::<OffersUniversityMapDto>(&text) {
                Ok(value) => {
                    log::info!(
                        "Offers <-> Institution list response success for {} speciality.",
                        speciality.code()
                    );
                    break value;
                },
                Err(_) => {
                    let error: ErrorResponse =
                        serde_json::from_str(&text).map_err(ApiError::JsonParseFailed)?;
                    error.handle_request_limit().await;
                },
            };
        };

        for dto in dto_map.universities {
            let value = OffersUniversity::try_from(dto)?;
            offers.push(value);
        }
    }

    Ok(offers)
}

pub struct OffersUniversitiesApi {
    pub qualification: Option<u16>,
    pub education_base: Option<u16>,
    pub speciality: Option<String>,
    pub region: Option<u16>,
    pub university: Option<u16>,
    pub study_program: Option<String>,
    pub education_form: Option<u16>,
    pub course: Option<u16>,
}

const QUALIFICATION: &str = "qualification";
const EDUCATION_BASE: &str = "education_base";
const SPECIALITY: &str = "speciality";
const REGION: &str = "region";
const UNIVERSITY: &str = "university";
const STUDY_PROGRAM: &str = "study_program";
const EDUCATION_FORM: &str = "education_form";
const COURSE: &str = "course";

impl ApiFetcherForm for OffersUniversitiesApi {
    fn create_form(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        Self::append_option_to_form(&mut map, QUALIFICATION, &self.qualification);
        Self::append_option_to_form(&mut map, EDUCATION_BASE, &self.education_base);
        Self::append_option_to_form(&mut map, SPECIALITY, &self.speciality);
        Self::append_option_to_form(&mut map, REGION, &self.region);
        Self::append_option_to_form(&mut map, UNIVERSITY, &self.university);
        Self::append_option_to_form(&mut map, STUDY_PROGRAM, &self.study_program);
        Self::append_option_to_form(&mut map, EDUCATION_FORM, &self.education_form);
        Self::append_option_to_form(&mut map, COURSE, &self.course);

        map
    }
}

impl ApiFetcherUrl for OffersUniversitiesApi {
    fn append_parameters_to_url(&self, url: &mut Url) {
        Self::append_optional_parameter(url, QUALIFICATION, &self.qualification);
        Self::append_optional_parameter(url, EDUCATION_BASE, &self.education_base);
        Self::append_optional_parameter(url, SPECIALITY, &self.speciality);
        Self::append_optional_parameter(url, REGION, &self.region);
        Self::append_optional_parameter(url, UNIVERSITY, &self.university);
        Self::append_optional_parameter(url, STUDY_PROGRAM, &self.study_program);
        Self::append_optional_parameter(url, EDUCATION_FORM, &self.education_form);
        Self::append_optional_parameter(url, COURSE, &self.course)
    }
}
