use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum StartCourse {
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
}

impl Display for StartCourse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::First => "1",
            Self::Second => "2",
            Self::Third => "3",
            Self::Fourth => "4",
            Self::Fifth => "5",
        };
        write!(f, "{value}")
    }
}
