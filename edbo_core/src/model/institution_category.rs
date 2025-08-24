use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, IntoPrimitive, TryFromPrimitive, EnumIter)]
#[repr(i8)]
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

    // UA: Заклад загальної середньої освіти (Unknown Code)
    GeneralSecondaryEducation = 3,

    // UA: Інший заклад освіти, що надає професійну (професійно-технічну освіту) (Unknown Code)
    OtherVocationalTechnical = 4,

    // (Unknown Code)
    Unknown = 5,
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

impl Display for InstitutionCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::HigherEducation => "Заклади вищої освіти",
            Self::VocationalTechnical => {
                "Заклади професійної (професійно-технічної) освіти"
            },
            Self::PreUniversityProfessional => "Заклади фахової передвищої освіти",
            Self::Scientific => "Наукові інститути (установи)",
            Self::Postgraduate => "Заклади післядипломної освіти",
            Self::GeneralSecondaryEducation => "Заклад загальної середньої освіти",
            Self::OtherVocationalTechnical => {
                "Інший заклад освіти, що надає професійну (професійно-технічну освіту)"
            },
            Self::Unknown => "Невідомо",
        };
        write!(f, "{}", text)
    }
}

impl InstitutionCategory {
    pub const fn code(&self) -> Option<i16> {
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
