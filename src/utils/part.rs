use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParsePartError;

impl fmt::Display for ParsePartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid part specified. Must be '1' or '2'.")
    }
}

impl std::error::Error for ParsePartError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Part {
    Part1,
    Part2,
}

impl FromStr for Part {
    type Err = ParsePartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_s = s.trim().to_lowercase();

        match lower_s.as_str() {
            "1" | "p1" | "part1" => Ok(Part::Part1),
            "2" | "p2" | "part2" => Ok(Part::Part2),
            _ => Err(ParsePartError),
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::Part1 => write!(f, "1"),
            Part::Part2 => write!(f, "2"),
        }
    }
}
