use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
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
            OwnershipForm::State => "Державна",
            OwnershipForm::Municipal => "Комунальна",
            OwnershipForm::Corporate => "Корпоративна",
            OwnershipForm::Private => "Приватна",
            OwnershipForm::Unknown => "Не визначено",
        };
        write!(f, "{s}")
    }
}
