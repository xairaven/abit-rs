use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Degree {
    // UA: Базова середня освіта
    LowerSecondary = 1,

    // UA: Кваліфікований робітник
    QualifiedWorker = 2,

    // UA: Повна загальна середня освіта
    HighSchool = 3,

    // UA: Бакалавр
    Bachelor = 4,

    // UA: Магістр
    Master = 5,

    // UA: Фаховий молодший бакалавр
    ProfessionalJuniorBachelor = 6,

    // UA: Молодший бакалавр
    JuniorBachelor = 7,

    // UA: Молодший спеціаліст
    JuniorSpecialist = 8,

    // UA: Доктор філософії
    DoctorOfPhilosophy = 9,

    // UA: Доктор мистецтв
    DoctorOfArts = 10,
}

impl Degree {
    pub fn qualification(&self) -> Result<u16, DegreeError> {
        match self {
            Self::Bachelor => Ok(1),
            Self::Master => Ok(2),
            Self::ProfessionalJuniorBachelor => Ok(9),
            Self::DoctorOfPhilosophy => Ok(7),
            Self::DoctorOfArts => Ok(10),
            _ => Err(DegreeError::QualificationCodeAbsent(self.to_string())),
        }
    }

    pub fn education_base(&self) -> Result<u16, DegreeError> {
        match self {
            Self::HighSchool => Ok(40),
            Self::Bachelor => Ok(620),
            Self::Master => Ok(640),
            Self::ProfessionalJuniorBachelor => Ok(530),
            Self::JuniorBachelor => Ok(610),
            Self::JuniorSpecialist => Ok(520),
            Self::LowerSecondary => Ok(30),
            Self::QualifiedWorker => Ok(510),
            _ => Err(DegreeError::EducationBaseCodeAbsent(self.to_string())),
        }
    }

    /// Possible base by qualification
    pub fn possible_bases(&self) -> Option<Vec<Self>> {
        match self {
            Self::Bachelor | Self::Master => Some(vec![
                Self::Bachelor,
                Self::Master,
                Self::HighSchool,
                Self::ProfessionalJuniorBachelor,
                Self::JuniorBachelor,
                Self::JuniorSpecialist,
            ]),
            Self::ProfessionalJuniorBachelor => Some(vec![
                Self::LowerSecondary,
                Self::HighSchool,
                Self::ProfessionalJuniorBachelor,
                Self::QualifiedWorker,
                Self::JuniorSpecialist,
            ]),
            Self::DoctorOfPhilosophy | Self::DoctorOfArts => Some(vec![Self::Master]),
            _ => None,
        }
    }
}

impl Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::LowerSecondary => "Базова середня освіта",
            Self::QualifiedWorker => "Кваліфікований робітник",
            Self::HighSchool => "Повна загальна середня освіта",
            Self::Bachelor => "Бакалавр",
            Self::Master => "Магістр",
            Self::ProfessionalJuniorBachelor => "Фаховий молодший бакалавр",
            Self::JuniorBachelor => "Молодший бакалавр",
            Self::JuniorSpecialist => "Молодший спеціаліст",
            Self::DoctorOfPhilosophy => "Доктор філософії",
            Self::DoctorOfArts => "Доктор мистецтв",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Error)]
pub enum DegreeError {
    #[error("Degree \"{0}\"does not have qualification code")]
    QualificationCodeAbsent(String),

    #[error("Degree \"{0}\"does not have education base code")]
    EducationBaseCodeAbsent(String),
}
