use crate::env::EnvError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Dotenv loading error.")]
    Env(#[from] EnvError),
}
