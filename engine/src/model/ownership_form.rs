use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, IntoPrimitive, TryFromPrimitive, EnumIter)]
#[repr(i8)]
pub enum OwnershipForm {
    State = 1,
    Municipal = 2,
    Corporate = 3,
    Private = 4,
    Unknown = 5,
}

impl From<&str> for OwnershipForm {
    fn from(value: &str) -> Self {
        match value {
            "Державна" => Self::State,
            "Комунальна" => Self::Municipal,
            "Корпоративна" => Self::Corporate,
            "Приватна" => Self::Private,
            _ => Self::Unknown,
        }
    }
}

impl Display for OwnershipForm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::State => "Державна",
            Self::Municipal => "Комунальна",
            Self::Corporate => "Корпоративна",
            Self::Private => "Приватна",
            Self::Unknown => "Не визначено",
        };
        write!(f, "{s}")
    }
}
