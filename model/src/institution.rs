use crate::institution::category::InstitutionCategory;
use crate::institution::ownership::OwnershipForm;
use crate::region::Region;

pub mod category;
pub mod ownership;

#[derive(Debug)]
pub struct Institution {
    pub title: String,
    pub id: u16,
    pub parent_id: Option<u16>,
    pub short_name: Option<String>,
    pub english_name: Option<String>,
    pub is_from_crimea: bool,
    pub registration_date: Option<String>,
    pub category: InstitutionCategory,
    pub ownership_form: OwnershipForm,
    pub region: Option<Region>,
}
