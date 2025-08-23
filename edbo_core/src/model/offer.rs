use crate::model::speciality::Speciality;
use crate::model::study_form::StudyForm;

#[derive(Debug)]
pub struct Offer {
    pub id: u32,
    pub title: String,
    pub education_program: String,
    pub faculty: String,
    pub speciality: Speciality,
    pub master_type: String,
    pub license_volume: i32,
    pub study_form: StudyForm,
    pub budgetary_places: i32,
}
