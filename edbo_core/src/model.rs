use crate::model::institution::InstitutionError;
use crate::model::region::RegionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Institution Error. {0}")]
    Institution(#[from] InstitutionError),

    #[error("Region Error. {0}")]
    Region(#[from] RegionError),
}

pub mod degree;
pub mod institution;
pub mod region;
pub mod speciality;
