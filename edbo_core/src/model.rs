use crate::model::degree::DegreeError;
use crate::model::institution::InstitutionError;
use crate::model::offers_university::OffersUniversityError;
use crate::model::region::RegionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Degree Error. {0}")]
    Degree(#[from] DegreeError),

    #[error("Institution Error. {0}")]
    Institution(#[from] InstitutionError),

    #[error("Region Error. {0}")]
    Region(#[from] RegionError),

    #[error("Offers <-> University Error. {0}")]
    OffersUniversityError(#[from] OffersUniversityError),
}

pub mod course;
pub mod degree;
pub mod institution;
pub mod offers_university;
pub mod region;
pub mod speciality;
pub mod study_form;
