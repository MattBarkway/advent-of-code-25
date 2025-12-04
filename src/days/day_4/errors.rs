use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[error("Invalid bank. should be a string of at least 2 positive integers")]
pub struct ParseGridError;
