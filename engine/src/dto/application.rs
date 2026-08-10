use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApplyRequestDtoMap {
    pub requests: Vec<ApplyRequestDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ApplyRequestDto {
    // Number of apply request
    pub n: i32,
    // Application status
    pub prsid: i32,
    // ???
    pub ptid: i32,
    // Encrypted Surname, Name, Parental
    pub fio: String,
    // Unknown identification field
    pub pa: i32,
    // Unknown identification field
    pub d: i32,
    // Unknown identification field
    pub cp: i32,
    // Verification of study place
    pub cpt: String,
    // ???
    pub cpd: String,
    // ???
    pub artid: i32,
    // Overall grade
    pub kv: f32,
    // Encrypted priority
    pub p: String,
    // Grade components (exams)
    pub rss: Vec<GradeComponentDto>,
}

#[derive(Serialize, Deserialize)]
pub struct GradeComponentDto {
    // Calculated component (for example, "+26.800")
    pub kv: String,
    // Formula of calculation (for example, "134 x 0.2")
    pub f: String,
    // Some identification field. Unique for each application
    pub id: i32,
}
