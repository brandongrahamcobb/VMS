use crate::config::error::ConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Requested world was not found in world model layer: {0}")]
    NotFound(i16),

    #[error("Config error in world model layer")]
    ConfigError(#[from] ConfigError),

    #[error("No worlds error in world model layer")]
    NoWorlds,
}
