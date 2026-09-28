use crate::error::RorschachError;
use thiserror::Error;

#[derive(Debug, Error)]

pub enum ConfigError {
    #[error("Failed to load configuration: {0}")]
    Config(#[from] figment::Error),
    #[error("Failed to convert questionnaire configuration: {0}")]
    ConversionError(#[from] RorschachError),
}
