use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstitutionError {
    #[error("Category. {0}")]
    Category(strum::ParseError),

    #[error("Ownership Form. {0}")]
    OwnershipForm(strum::ParseError),

    #[error("Region. {0}")]
    Region(strum::ParseError),
}
