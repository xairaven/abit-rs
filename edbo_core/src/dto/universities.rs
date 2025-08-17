use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UniversityDto {
    pub university_name: String,
    pub university_id: String,
    pub university_parent_id: Option<String>,
    pub university_short_name: Option<String>,
    pub university_name_en: Option<String>,
    pub is_from_crimea: Option<String>,
    pub registration_year: Option<String>,
    pub university_type_name: String,
    pub university_financing_type_name: String,
    pub university_governance_type_name: Option<String>,
    pub post_index_u: String,
    pub katottgcodeu: String,
    pub region_name_u: String,
    pub katottg_name_u: String,
    pub university_address_u: String,
    pub university_phone: Option<String>,
    pub university_email: Option<String>,
    pub university_site: Option<String>,
    pub university_director_post: Option<String>,
    pub university_director_fio: Option<String>,
    pub close_date: Option<String>,
    pub primitki: Option<String>,
}
