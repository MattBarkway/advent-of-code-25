use crate::days::day_1::errors::ParseMoveError;
use std::io;

#[derive(Debug)]
pub enum RunError {
    Io(io::Error),
    Parse(ParseMoveError),
}

impl From<io::Error> for RunError {
    fn from(err: io::Error) -> Self {
        RunError::Io(err)
    }
}
impl From<ParseMoveError> for RunError {
    fn from(err: ParseMoveError) -> Self {
        RunError::Parse(err)
    }
}
