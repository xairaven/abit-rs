#[derive(Debug)]
pub enum InstitutionCategory {
    // UA: Заклади вищої освіти
    HigherEducation = 1,

    // UA: Заклади професійної (професійно-технічної) освіти
    VocationalTechnical = 2,

    // UA: Заклади фахової передвищої освіти
    PreUniversityProfessional = 9,

    // UA: Наукові інститути (установи)
    Scientific = 8,

    // UA: Заклади післядипломної освіти
    Postgraduate = 10,

    // UA: Заклад загальної середньої освіти
    GeneralSecondaryEducation,

    // UA: Інший заклад освіти, що надає професійну (професійно-технічну освіту)
    OtherVocationalTechnical,

    Unknown,
}

impl From<&str> for InstitutionCategory {
    fn from(value: &str) -> Self {
        match value {
            "Заклад вищої освіти" => Self::HigherEducation,
            "Заклад загальної середньої освіти" => {
                Self::GeneralSecondaryEducation
            },
            "Заклад післядипломної освіти" => {
                Self::Postgraduate
            },
            "Заклад професійної (професійно-технічної) освіти" => {
                Self::VocationalTechnical
            },
            "Заклад фахової передвищої освіти" => {
                Self::PreUniversityProfessional
            },
            "Інший заклад освіти, що надає професійну (професійно-технічну освіту)" => {
                Self::OtherVocationalTechnical
            },
            "Наукові інститути (установи)" => Self::Scientific,
            _ => Self::Unknown,
        }
    }
}

impl InstitutionCategory {
    pub const fn code(&self) -> Option<u16> {
        match self {
            Self::HigherEducation => Some(1),
            Self::VocationalTechnical => Some(2),
            Self::PreUniversityProfessional => Some(9),
            Self::Scientific => Some(8),
            Self::Postgraduate => Some(10),
            _ => None,
        }
    }
}
