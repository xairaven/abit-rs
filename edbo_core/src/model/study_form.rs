use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum StudyForm {
    FullTime = 1,
    External = 2,
    Evening = 4,
    Online = 5,
}

impl TryFrom<&str> for StudyForm {
    type Error = StudyFormError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Денна" => Ok(Self::FullTime),
            "Заочна" => Ok(Self::External),
            "Вечірня" => Ok(Self::Evening),
            "Дистанційна" => Ok(Self::Online),
            _ => Err(Self::Error::UnknownForm(value.to_string())),
        }
    }
}

impl std::fmt::Display for StudyForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::FullTime => "Денна",
            Self::External => "Заочна",
            Self::Evening => "Вечірня",
            Self::Online => "Дистанційна",
        };

        write!(f, "{}", text)
    }
}

#[derive(Debug, Error)]
pub enum StudyFormError {
    #[error("Unknown form: {0}")]
    UnknownForm(String),
}
