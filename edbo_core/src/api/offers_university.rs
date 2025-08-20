use crate::api;
use crate::api::{ApiError, ApiFetcher};
use crate::dto::offers_university::OffersUniversityDto;
use crate::error::CoreError;
use crate::model::degree::Degree;
use crate::model::offers_university::OffersUniversity;
use crate::model::{ModelError, speciality};
use url::Url;

pub async fn list() -> Result<Vec<OffersUniversity>, CoreError> {
    type Error = ApiError;

    let base_url = format!("{}/offers-universities/", api::links::MAIN);
    let url = Url::parse(&base_url).map_err(Error::FailedToParseUrl)?;

    let client = reqwest::Client::builder()
        .build()
        .map_err(Error::FailedBuildClient)?;

    let mut parameters = OffersUniversitiesApi {
        qualification: Some(Degree::Master.qualification().map_err(ModelError::Degree)?),
        education_base: Some(
            Degree::Bachelor
                .qualification()
                .map_err(ModelError::Degree)?,
        ),
        speciality: None,
        region: None,
        university: None,
        study_program: None,
        education_form: None,
        course: None,
    };

    const INTERVAL_FOR_REQUESTS: tokio::time::Duration =
        tokio::time::Duration::from_millis(500);
    let mut ticker = tokio::time::interval(INTERVAL_FOR_REQUESTS);

    let mut offers: Vec<OffersUniversity> = vec![];
    for (_, speciality) in speciality::ALL_SPECIALITIES.iter() {
        parameters.speciality = Some(speciality.code().to_string());
        let mut url = url.clone();
        parameters.append_parameters_to_url(&mut url);

        ticker.tick().await;

        let response = client
            .post(url)
            .send()
            .await
            .map_err(Error::RequestFailed)?;
        log::info!(
            "Offers <-> Institution list response success for {} speciality.",
            speciality.code()
        );

        let text = response
            .text()
            .await
            .map_err(Error::FailedToGetResponseText)?;
        log::debug!("Text from response: {:?}", text);
        let dto_list: Vec<OffersUniversityDto> =
            serde_json::from_str(&text).map_err(Error::JsonParseFailed)?;

        for dto in dto_list {
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

impl ApiFetcher for OffersUniversitiesApi {
    fn append_parameters_to_url(&self, url: &mut Url) {
        const QUALIFICATION: &str = "qualification";
        const EDUCATION_BASE: &str = "education_base";
        const SPECIALITY: &str = "speciality";
        const REGION: &str = "region";
        const UNIVERSITY: &str = "university";
        const STUDY_PROGRAM: &str = "study_program";
        const EDUCATION_FORM: &str = "education_form";
        const COURSE: &str = "course";

        Self::append_optional_parameter(url, QUALIFICATION, &self.qualification);
        Self::append_optional_parameter(url, EDUCATION_BASE, &self.education_base);
        Self::append_optional_parameter(url, SPECIALITY, &self.speciality);
        Self::append_optional_parameter(url, REGION, &self.region);
        Self::append_optional_parameter(url, UNIVERSITY, &self.university);
        Self::append_optional_parameter(url, STUDY_PROGRAM, &self.study_program);
        Self::append_optional_parameter(url, EDUCATION_FORM, &self.education_form);
        Self::append_optional_parameter(url, COURSE, &self.course);
    }
}
