use thiserror::Error;

#[derive(Debug)]
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

#[derive(Debug, Error)]
pub enum OfferTypeError {
    #[error("Invalid offer type: {0}")]
    InvalidType(String),
}
