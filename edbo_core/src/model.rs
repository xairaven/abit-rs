use crate::model::degree::DegreeError;
use crate::model::institution::InstitutionError;
use crate::model::offer_type::OfferTypeError;
use crate::model::offers_university::OffersUniversityError;
use crate::model::region::RegionError;
use crate::model::speciality::SpecialityError;
use crate::model::status::ApplicationStatusError;
use crate::model::study_form::StudyFormError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Application Status Error. {0}")]
    ApplicationStatus(#[from] ApplicationStatusError),

    #[error("Degree Error. {0}")]
    Degree(#[from] DegreeError),

    #[error("Institution Error. {0}")]
    Institution(#[from] InstitutionError),

    #[error("Offers <-> University Error. {0}")]
    OffersUniversityError(#[from] OffersUniversityError),

    #[error("Offer Type Error. {0}")]
    OfferType(#[from] OfferTypeError),

    #[error("Region Error. {0}")]
    Region(#[from] RegionError),

    #[error("Speciality Error. {0}")]
    Speciality(#[from] SpecialityError),

    #[error("Study Form Error. {0}")]
    StudyForm(#[from] StudyFormError),
}

pub mod course;
pub mod degree;
pub mod institution;
pub mod offer;
pub mod offer_type;
pub mod offers_university;
pub mod region;
pub mod speciality;
pub mod status;
pub mod study_form;
