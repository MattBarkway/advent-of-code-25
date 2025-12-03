use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseDayError;

impl fmt::Display for ParseDayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid day specified. Must be between '1' and '12'.")
    }
}

impl std::error::Error for ParseDayError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Day {
    Day1,
    Day2,
    Day3,
}

impl FromStr for Day {
    type Err = ParseDayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_s = s.trim().to_lowercase();

        match lower_s.as_str() {
            "1" | "day1" | "day 1" => Ok(Day::Day1),
            "2" | "day2" | "day 2" => Ok(Day::Day2),
            "3" | "day3" | "day 3" => Ok(Day::Day3),
            _ => Err(ParseDayError),
        }
    }
}
