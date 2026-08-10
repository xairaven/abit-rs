use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OffersUniversityMapDto {
    pub universities: Vec<OffersUniversityDto>,
}

#[derive(Debug, Deserialize)]
pub struct OffersUniversityDto {
    pub uid: i32,
    pub un: String,
    pub ids: String,
    pub n: i32,
}
