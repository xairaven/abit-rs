use crate::model::degree::Degree;
use crate::model::offer_type::OfferType;
use crate::model::speciality::Speciality;
use crate::model::study_form::StudyForm;

#[derive(Debug)]
pub struct Offer {
    pub id: i32,
    pub title: String,
    pub degree: Degree,
    pub education_program: String,
    pub faculty: Option<String>,
    pub speciality: Speciality,
    pub funding_type: OfferType,
    pub master_type: Option<String>,
    pub study_form: StudyForm,
    pub license_volume: i32,
    pub budgetary_places: i32,
}
