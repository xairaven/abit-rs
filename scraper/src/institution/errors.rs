use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstitutionError {
    // DTO Parsing
    #[error("Category DTO Parsing. {0}")]
    CategoryDto(strum::ParseError),

    #[error("Ownership Form DTO Parsing. {0}")]
    OwnershipFormDto(strum::ParseError),

    #[error("Region DTO Parsing. {0}")]
    RegionDto(strum::ParseError),

    // Serializing
    #[error("Deserializing. {0}")]
    Deserializing(serde_json::Error),

    // API
    #[error("Request. {0}")]
    Request(reqwest::Error),

    #[error("Request Text. {0}")]
    RequestText(reqwest::Error),

    // SQL
    #[error("Find All query. {0}")]
    FindAll(sqlx::Error),

    #[error("Insert query. {0}")]
    Insert(sqlx::Error),

    #[error("Is table empty check. {0}")]
    IsEmpty(sqlx::Error),

    #[error("Inconsistent dictionary data. {0}")]
    InconsistentDictionaryData(String),
}
