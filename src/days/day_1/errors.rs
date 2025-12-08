use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[error("Invalid move specified. Must be either R or L.")]
pub struct ParseDirectionError;

#[derive(Debug, Error)]
pub enum ParseMoveError {
    /// Error when the direction fails to parse
    #[error("Direction parsing failed: {0}")]
    InvalidDirection(#[from] ParseDirectionError),
    /// Error when the number fails to parse
    #[error("Step parsing failed: {0}")]
    InvalidSteps(#[from] ParseIntError),
}
