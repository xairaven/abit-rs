use thiserror::Error;

pub mod institution;

#[derive(Debug, Error)]
pub enum DtoError {
    #[error("Institution Category. {0}")]
    InstitutionCategory(strum::ParseError),

    #[error("Ownership Form. {0}")]
    OwnershipForm(strum::ParseError),

    #[error("Region. {0}")]
    Region(strum::ParseError),
}
