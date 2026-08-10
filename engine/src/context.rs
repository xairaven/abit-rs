use crate::model::applicant::Applicant;
use crate::model::application::Application;
use crate::model::institution::Institution;
use crate::model::offer::Offer;
use crate::model::offers_university::OffersUniversity;

pub struct Context {
    pub applicants: Vec<Applicant>,
    pub applications: Vec<Application>,
    pub institutions: Vec<Institution>,
    pub offers: Vec<Offer>,
    pub offers_with_institutions: Vec<OffersUniversity>,
}
