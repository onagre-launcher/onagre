use super::Rule;
use pest::error::Error as PestError;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to open config file")]
    IO(#[from] io::Error),
    #[error("Failed to parse config file:\n{0}")]
    Parse(#[from] Box<PestError<Rule>>),
    #[error("Failed to parse number")]
    ParseInt(#[from] ParseIntError),
    #[error("Failed to parse number")]
    ParseFloat(#[from] ParseFloatError),
    #[error("Failed to parse '{0}' as hex color")]
    ParseColor(String),
}
