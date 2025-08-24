use crate::crypto;
use crate::dto::application::{ApplyRequestDto, GradeComponentDto};
use crate::model::ModelError;
use crate::model::applicant::Applicants;
use crate::model::priority::Priority;
use crate::model::status::ApplicationStatus;
use thiserror::Error;

#[derive(Debug)]
pub struct Application {
    pub number_in_list: i32,
    pub status: ApplicationStatus,
    pub grade: f32,
    pub priority: Priority,

    pub offer_id: i32,
    pub user_id: i32,
}

impl Application {
    pub fn try_from_dto(
        dto: ApplyRequestDto, offer_id: i32, applicants: &mut Applicants,
    ) -> Result<Self, ModelError> {
        let number_in_list = dto.n;
        let status = ApplicationStatus::try_from(dto.prsid)?;
        let full_name = crypto::decrypt(dto.fio, number_in_list, dto.prsid)?;
        let grade = dto.kv;
        let priority = Priority::try_from(
            crypto::decrypt(dto.p, number_in_list, dto.prsid)?.as_str(),
        )?;
        let mut grade_components: Vec<GradeComponent> = vec![];
        for component_dto in dto.rss {
            let component = GradeComponent::try_from(component_dto)?;
            grade_components.push(component);
        }

        let user_id = applicants.add_application(full_name, grade_components)?;

        Ok(Self {
            offer_id,
            number_in_list,
            status,
            grade,
            priority,

            user_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GradeComponent(pub f32);

impl TryFrom<GradeComponentDto> for GradeComponent {
    type Error = ModelError;

    fn try_from(dto: GradeComponentDto) -> Result<Self, Self::Error> {
        let grade = dto
            .kv
            .split(' ')
            .collect::<Vec<&str>>()
            .first()
            .ok_or(GradeComponentError::FailedToSplit(dto.kv.to_string()))?
            .parse::<f32>()
            .map_err(GradeComponentError::FailedToParse)?;

        Ok(Self(grade))
    }
}

impl PartialEq for GradeComponent {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 0.0001
    }
}

#[derive(Debug, Error)]
pub enum GradeComponentError {
    #[error("Failed to split grade: {0}")]
    FailedToSplit(String),

    #[error("Failed to parse grade: {0}")]
    FailedToParse(#[from] std::num::ParseFloatError),
}
