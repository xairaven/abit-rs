use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub enum Priority {
    Budgetary(i8),
    Contract,
}

impl TryFrom<&str> for Priority {
    type Error = PriorityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim() == "(К)" {
            return Ok(Priority::Contract);
        }

        let parts = value.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(Self::Error::UnknownValue(String::from(value)));
        }
        let number = parts
            .first()
            .ok_or(Self::Error::UnknownValue(String::from(value)))?
            .parse::<i8>()
            .map_err(|_| Self::Error::UnknownValue(String::from(value)))?;
        if !parts
            .get(1)
            .ok_or(Self::Error::UnknownValue(String::from(value)))?
            .eq(&"(Б)")
        {
            return Err(Self::Error::UnknownValue(String::from(value)));
        }

        Ok(Priority::Budgetary(number))
    }
}

impl From<i8> for Priority {
    fn from(value: i8) -> Self {
        match value {
            0 => Priority::Contract,
            _ => Priority::Budgetary(value),
        }
    }
}

impl From<Priority> for i8 {
    fn from(value: Priority) -> Self {
        match value {
            Priority::Budgetary(number) => number,
            Priority::Contract => 0,
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Budgetary(number) => format!("{} (Б)", number),
            Self::Contract => "(К)".to_string(),
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Error)]
pub enum PriorityError {
    #[error("Failed to convert priority value. {0}")]
    UnknownValue(String),

    #[error("Unknown priority code: {0}")]
    UnknownCode(i32),
}
