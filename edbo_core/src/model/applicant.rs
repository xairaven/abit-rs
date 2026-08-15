use crate::dto::application::GradeComponentDto;
use crate::model::ModelError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug)]
pub struct Applicant {
    pub id: i32,
    pub name: String,
    pub grade_components: Vec<GradeComponent>,
}

#[derive(Debug, Default)]
pub struct Applicants {
    pub applicants: HashMap<String, Vec<Applicant>>,
    pub id_counter: i32,
}

impl Applicants {
    pub fn add_application(
        &mut self, full_name: String, grade_components: Vec<GradeComponent>,
    ) -> Result<i32, ApplicantError> {
        let applicants = self.applicants.entry(full_name.clone()).or_default();

        let mut user_index: Option<usize> = None;
        for (index, applicant) in applicants.iter().enumerate() {
            if Self::is_same_person_by_grades(&grade_components, applicant) {
                user_index = Some(index);
                break;
            }
        }

        match user_index {
            Some(index) => {
                let applicant = applicants
                    .get_mut(index)
                    .ok_or(ApplicantError::FailedToIndexApplicant(full_name, index))?;
                for grade in grade_components {
                    if !applicant.grade_components.contains(&grade) {
                        applicant.grade_components.push(grade);
                    }
                }
                Ok(applicant.id)
            },
            None => {
                let id = self.id_counter;
                self.id_counter += 1;

                let new_applicant = Applicant {
                    id,
                    name: full_name,
                    grade_components,
                };
                applicants.push(new_applicant);
                Ok(id)
            },
        }
    }

    pub fn to_vec(self) -> Vec<Applicant> {
        let mut applicants: Vec<Applicant> = Vec::new();
        for mut values in self.applicants.into_values() {
            applicants.append(&mut values);
        }
        applicants
    }

    fn is_same_person_by_grades(
        grades: &[GradeComponent], applicant: &Applicant,
    ) -> bool {
        const MUST_EQUAL: usize = 2;
        let mut equal_count = 0;
        let mut exclude_indexes: Vec<usize> = Vec::new();
        for grade_person in &applicant.grade_components {
            for (i, grade_application) in grades.iter().enumerate() {
                if exclude_indexes.contains(&i) {
                    continue;
                }
                if grade_person.0 == grade_application.0 {
                    equal_count += 1;
                    exclude_indexes.push(i);
                    break;
                }
            }
        }

        MUST_EQUAL >= equal_count
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    #[error("Failed to convert big int to float")]
    FailedFromBigInt,

    #[error("Failed to convert float to big decimal")]
    FailedToBigDecimal(#[from] bigdecimal::ParseBigDecimalError),
}

#[derive(Debug, Error)]
pub enum ApplicantError {
    #[error("Failed to index applicant \"{0}\" at index {1}")]
    FailedToIndexApplicant(String, usize),
}
