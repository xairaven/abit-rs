use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumIter;
use thiserror::Error;

#[derive(Debug, Copy, Clone, IntoPrimitive, TryFromPrimitive, EnumIter)]
#[repr(i8)]
pub enum OfferType {
    Open = 1,
    Fixed = 2,
    NonBudgetary = 3,
}

impl TryFrom<&str> for OfferType {
    type Error = OfferTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Відкрита" => Ok(Self::Open),
            "Фіксована" => Ok(Self::Fixed),
            "Небюджетна" => Ok(Self::NonBudgetary),
            _ => Err(Self::Error::InvalidType(value.to_string())),
        }
    }
}

impl std::fmt::Display for OfferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Open => "Відкрита",
            Self::Fixed => "Фіксована",
            Self::NonBudgetary => "Небюджетна",
        };

        write!(f, "{}", text)
    }
}

#[derive(Debug, Error)]
pub enum OfferTypeError {
    #[error("Invalid offer type: {0}")]
    InvalidType(String),

    #[error("Invalid offer type ID: {0}")]
    InvalidCode(i8),
}
