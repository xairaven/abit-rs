use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(i8)]
pub enum InstitutionCategory {
    HigherEducation = 1,
    ProfessionalCollege = 9,
    VocationalEducation = 2,
    SecondaryEducation = 3,
    ScientificInstitutes = 8,
    Postgrad = 10,

    OtherVET = 4,
    Unknown = 5,
}