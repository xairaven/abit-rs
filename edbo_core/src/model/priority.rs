use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Priority {
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,

    // Unknown code
    Contract = 6,
}

impl TryFrom<&str> for Priority {
    type Error = PriorityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "1 (Б)" => Ok(Priority::First),
            "2 (Б)" => Ok(Priority::Second),
            "3 (Б)" => Ok(Priority::Third),
            "4 (Б)" => Ok(Priority::Fourth),
            "5 (Б)" => Ok(Priority::Fifth),
            "(К)" => Ok(Priority::Contract),
            _ => Err(Self::Error::UnknownValue(String::from(value))),
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Priority::First => "1 (Б)",
            Priority::Second => "2 (Б)",
            Priority::Third => "3 (Б)",
            Priority::Fourth => "4 (Б)",
            Priority::Fifth => "5 (Б)",
            Priority::Contract => "(К)",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Error)]
pub enum PriorityError {
    #[error("Failed to convert priority value. {0}")]
    UnknownValue(String),
}
