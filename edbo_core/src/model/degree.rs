#[derive(Debug)]
pub enum Degree {
    // UA: Базова середня освіта
    LowerSecondary,

    // UA: Кваліфікований робітник
    QualifiedWorker,

    // UA: Повна загальна середня освіта
    HighSchool,

    // UA: Бакалавр
    Bachelor,

    // UA: Магістр
    Master,

    // UA: Фаховий молодший бакалавр
    ProfessionalJuniorBachelor,

    // UA: Молодший бакалавр
    JuniorBachelor,

    // UA: Молодший спеціаліст
    JuniorSpecialist,

    // UA: Доктор філософії
    DoctorOfPhilosophy,

    // UA: Доктор мистецтв
    DoctorOfArts,
}

impl Degree {
    pub fn qualification(&self) -> Option<u16> {
        match self {
            Self::Bachelor => Some(1),
            Self::Master => Some(2),
            Self::ProfessionalJuniorBachelor => Some(9),
            Self::DoctorOfPhilosophy => Some(7),
            Self::DoctorOfArts => Some(10),
            _ => None,
        }
    }

    pub fn education_base(&self) -> Option<u16> {
        match self {
            Self::HighSchool => Some(40),
            Self::Bachelor => Some(620),
            Self::Master => Some(640),
            Self::ProfessionalJuniorBachelor => Some(530),
            Self::JuniorBachelor => Some(610),
            Self::JuniorSpecialist => Some(520),
            Self::LowerSecondary => Some(30),
            Self::QualifiedWorker => Some(510),
            _ => None,
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
