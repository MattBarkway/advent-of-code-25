use crate::utils::advent_day::AdventDay;
use crate::utils::grid::{Grid, ParseGridError};
use crate::utils::load::raw_load_from_file;
use anyhow::{Result};
use itertools::Itertools;
use std::str::{FromStr};
use thiserror::Error;

pub struct DaySix;

impl AdventDay for DaySix {
    fn part_1(&self) -> Result<()> {
        tracing::info!("Day 6: Part 1");
        let input = raw_load_from_file("inputs/day_6/part1.txt")?;
        let sum = do_math_homework(&input)?;
        tracing::info!("Calculated homework sum as: {}", sum);
        Ok(())
    }

    fn part_2(&self) -> Result<()> {
        tracing::info!("Day 6: Part 2");
        let input = raw_load_from_file("inputs/day_6/part1.txt")?;
        let sum = do_math_homework_pt2(&input)?;

        tracing::info!("Calculated homework sum as: {}", sum);
        Ok(())
    }
}

impl FromStr for Grid<String> {
    type Err = ParseGridError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|row| row.split_whitespace().map(|s| s.to_string()).collect_vec())
            .collect_vec();
        Ok(Grid::new(rows[0].len(), rows.clone()))
    }
}

#[derive(Debug, Error, Clone)]
#[error("Invalid MathColumn. should be a Vec of string numbers, with the final element being the operation")]
pub struct ParseMathColumnError;

struct MathColumn {
    numbers: Vec<i64>,
    operation: char,
}

impl MathColumn {
    fn new(numbers: Vec<i64>, operation: char) -> Self {
        assert!(numbers.len() > 1);
        assert!(['+', '-', '*', '/'].contains(&operation));
        Self { numbers, operation }
    }
}

impl TryFrom<Vec<String>> for MathColumn {
    type Error = ParseMathColumnError;

    fn try_from(value: Vec<String>) -> std::result::Result<Self, Self::Error> {
        let (operation, raw_numbers) = value.split_last().ok_or(ParseMathColumnError)?;
        let numbers = raw_numbers
            .iter()
            .map(|s| s.parse::<i64>().map_err(|_| ParseMathColumnError))
            .collect::<Result<_, _>>()?;
        Ok(Self::new(
            numbers,
            operation.chars().next().ok_or(ParseMathColumnError)?,
        ))
    }
}

impl MathColumn {
    fn calculate(&self) -> Option<i64> {
        self.numbers
            .clone()
            .into_iter()
            .reduce(operation_from_char(self.operation))
    }
}

fn operation_from_char(char: char) -> impl Fn(i64, i64) -> i64 {
    if char == '+' {
        |a, b| a + b
    } else if char == '-' {
        |a, b| a - b
    } else if char == '*' {
        |a, b| a * b
    } else {
        |a, b| a / b
    }
}

fn do_math_homework(sheet: &str) -> Result<i64> {
    let grid: Grid<String> = Grid::from_str(sheet)?;
    let transposed = grid.transpose("".into());
    let columns = transposed
        .values
        .into_iter()
        .map(MathColumn::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(columns
        .iter()
        .fold(0, |acc, col| acc + col.calculate().unwrap_or(0)))
}

fn do_math_homework_pt2(sheet: &str) -> Result<i64> {
    let chars: Vec<Vec<char>> = sheet
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let transposed = Grid {
        width: chars[0].len(),
        values: chars,
    }
    .transpose(' ');

    let columns = transposed
        .values
        .chunks(4)
        .map(|chunk| chunk.to_vec())
        .map(|mut rows| {
            let idx = rows[0].len() - 1;
            let op = rows[0][idx];
            rows[0][idx] = ' ';
            Ok(MathColumn::new(
                rows.iter()
                    .filter_map(|i| {
                        let s = i.iter().collect::<String>();
                        let x = s.trim();
                        if x.is_empty() {
                            None
                        } else {
                            Some(x.parse::<i64>())
                        }
                    })
                    .collect::<Result<_, _>>()?,
                op,
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(columns
        .iter()
        .fold(0, |acc, col| acc + col.calculate().unwrap_or(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_homework_part1() {
        let test_input = r"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

        let total = do_math_homework(test_input).unwrap();
        assert_eq!(total, 4277556);
    }

    #[test]
    fn test_do_homework_part2() {
        let test_input = [
            // retain trailing spaces:
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n");

        let total = do_math_homework_pt2(&test_input).unwrap();
        assert_eq!(total, 3263827);
    }
}
