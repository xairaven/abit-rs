use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;
use thiserror::Error;

#[derive(Debug, Copy, Clone, IntoPrimitive, TryFromPrimitive, EnumIter)]
#[repr(i8)]
pub enum ApplicationStatus {
    ApplicationReceived = 1,
    Pending = 2,
    CancelledByApplicant = 3,
    CancelledPriorityLost = 4,
    Registered = 5,
    Admitted = 6,
    Rejected = 7,
    CancelledByInstitution = 8,
    RecommendedBudget = 9,
    RejectedBudget = 10,
    AdmittedContractDecision = 11,
    RecommendedContract = 12,
    RejectedContract = 13,
    ToEnrollmentOrder = 14,
    Expelled = 15,
    DeactivatedEnrolled = 16,
}

impl Display for ApplicationStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::ApplicationReceived => "Заява надійшла з сайту",
            Self::Pending => "Затримано",
            Self::CancelledByApplicant => "Скасовано вступником",
            Self::CancelledPriorityLost => "Скасовано (втрата пріоритету)",
            Self::Registered => "Зареєстровано",
            Self::Admitted => "Допущено",
            Self::Rejected => "Відмова",
            Self::CancelledByInstitution => "Скасовано закладом освіти",
            Self::RecommendedBudget => "Рекомендовано (бюджет)",
            Self::RejectedBudget => "Відхилено (бюджет)",
            Self::AdmittedContractDecision => "Допущено (контракт, за ріш. ПК)",
            Self::RecommendedContract => "Рекомендовано (контракт)",
            Self::RejectedContract => "Відхилено (контракт)",
            Self::ToEnrollmentOrder => "До наказу",
            Self::Expelled => "Відраховано",
            Self::DeactivatedEnrolled => "Деактивовано (зараховано на навчання)",
        };

        write!(f, "{}", text)
    }
}

impl TryFrom<i32> for ApplicationStatus {
    type Error = ApplicationStatusError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ApplicationReceived),
            2 => Ok(Self::Pending),
            3 => Ok(Self::CancelledByApplicant),
            4 => Ok(Self::CancelledPriorityLost),
            5 => Ok(Self::Registered),
            6 => Ok(Self::Admitted),
            7 => Ok(Self::Rejected),
            8 => Ok(Self::CancelledByInstitution),
            9 => Ok(Self::RecommendedBudget),
            10 => Ok(Self::RejectedBudget),
            11 => Ok(Self::AdmittedContractDecision),
            12 => Ok(Self::RecommendedContract),
            13 => Ok(Self::RejectedContract),
            14 => Ok(Self::ToEnrollmentOrder),
            15 => Ok(Self::Expelled),
            16 => Ok(Self::DeactivatedEnrolled),
            _ => Err(Self::Error::UnknownCode(value)),
        }
    }
}

#[derive(Debug, Error)]
pub enum ApplicationStatusError {
    #[error("Unknown application status code.")]
    UnknownCode(i32),
}
