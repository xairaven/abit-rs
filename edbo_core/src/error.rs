use crate::env::EnvError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Configuration loading error: {0}")]
    Env(#[from] EnvError),
}
