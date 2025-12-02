use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseMoveError {
    /// Error when the direction fails to parse
    InvalidDirection(ParseDirectionError),
    /// Error when the number fails to parse
    InvalidSteps(ParseIntError),
    /// Error if the string is empty or not in the expected format
    InvalidFormat,
}

impl fmt::Display for ParseMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseMoveError::InvalidDirection(e) => write!(f, "Direction parsing failed: {}", e),
            ParseMoveError::InvalidSteps(e) => write!(f, "Step parsing failed: {}", e),
            ParseMoveError::InvalidFormat => {
                write!(f, "Move string must be in the format 'D#' (e.g., L99)")
            }
        }
    }
}
impl std::error::Error for ParseMoveError {}

impl From<ParseDirectionError> for ParseMoveError {
    fn from(err: ParseDirectionError) -> Self {
        ParseMoveError::InvalidDirection(err)
    }
}

impl From<ParseIntError> for ParseMoveError {
    fn from(err: ParseIntError) -> Self {
        ParseMoveError::InvalidSteps(err)
    }
}

#[derive(Debug, Clone)]
pub struct ParseDirectionError;

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid move specified. Must be either R or L.")
    }
}

impl std::error::Error for ParseDirectionError {}
