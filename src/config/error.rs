use super::Rule;
use pest::error::Error as PestError;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to open config file")]
    IOError(#[from] io::Error),
    #[error("Failed to parse config file:\n{0}")]
    ParseError(#[from] PestError<Rule>),
    #[error("Failed to parse number")]
    ParseIntError(#[from] ParseIntError),
    #[error("Failed to parse number")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Failed to parse '{0}' as hex color")]
    ParseColorError(String),
}
