use crate::error::CoreError;

pub fn process() -> Result<(), CoreError> {
    let config = env::from_env()?;
    dbg!(&config);

    Ok(())
}

mod env;
mod error;
