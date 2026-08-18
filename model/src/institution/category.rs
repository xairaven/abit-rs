use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, IntoPrimitive, TryFromPrimitive, EnumString, Display)]
#[repr(i16)]
pub enum InstitutionCategory {
    #[strum(serialize = "Заклад вищої освіти")]
    HigherEducation = 1,
    #[strum(serialize = "Заклади фахової передвищої освіти")]
    ProfessionalCollege = 9,
    #[strum(serialize = "Заклад професійної (професійно-технічної) освіти")]
    VocationalEducation = 2,
    #[strum(serialize = "Заклад загальної середньої освіти")]
    SecondaryEducation = 3,
    #[strum(serialize = "Наукові інститути (установи)")]
    ScientificInstitutes = 8,
    #[strum(serialize = "Заклади післядипломної освіти")]
    Postgrad = 10,

    #[strum(
        serialize = "Інший заклад освіти, що надає професійну (професійно-технічну освіту)"
    )]
    OtherVET = 4,
    #[strum(serialize = "")]
    Unknown = 5,
}
