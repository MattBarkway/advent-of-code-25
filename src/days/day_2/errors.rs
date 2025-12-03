use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[error("Invalid range. Both values should be positive integers.")]
pub struct ParseRangeError;

#[derive(Debug, Error)]
pub enum ParseProductError {
    /// Error when the direction fails to parse
    #[error("Product range parsing failed: {0}")]
    InvalidRange(#[from] ParseRangeError),
    /// Error when the number fails to parse
    #[error("Step parsing failed: {0}")]
    InvalidSteps(#[from] ParseIntError),
    /// Error if the string is empty or not in the expected format
    #[error("Each product must be two numbers separated by a hyphen, e.g., '1-5'")]
    InvalidFormat,
}
