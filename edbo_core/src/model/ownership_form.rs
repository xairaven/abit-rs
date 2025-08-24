use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
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
