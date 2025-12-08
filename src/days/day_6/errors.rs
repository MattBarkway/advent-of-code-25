use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[error("Invalid Grid. should be a rectangular grid of chars")]
pub struct ParseGridError;
