use crate::days::day_1::errors::{ParseDirectionError, ParseMoveError};
use crate::utils::advent_day::AdventDay;
use crate::utils::load::load_from_file;
use anyhow::Result;
use std::str::FromStr;

pub struct DayOne;

impl AdventDay for DayOne {
    fn part_1(&self) -> Result<()> {
        tracing::info!("Day 1: Part 1");
        let raw_moves = load_from_file("inputs/day_1/part1.txt")?;
        let cracker = SafeCracker::from_raw_inputs(50, raw_moves, 100)?;
        let count = cracker.count_zeros();
        tracing::info!("The password is {}", count);
        Ok(())
    }

    fn part_2(&self) -> Result<()> {
        println!("Day 1: Part 2");
        let raw_moves = load_from_file("inputs/day_1/part1.txt")?;
        let cracker = SafeCracker::from_raw_inputs(50, raw_moves, 100)?;
        let count = cracker.count_zero_incl_passes();
        tracing::info!("The password is {}", count);
        Ok(())
    }
}

// Inputs like L5 or R7
// Means rotate dial left or right that number of steps
// Dial goes from 0 to 99, and loops back to 0
// Dial starts at 50
// "The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence."

pub enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim().to_lowercase();
        match value.as_str() {
            "l" | "left" => Ok(Direction::Left),
            "r" | "right" => Ok(Direction::Right),
            _ => Err(ParseDirectionError),
        }
    }
}

struct Move {
    direction: Direction,
    steps: i32,
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (char_str, num_str) = s.split_at(1);
        let direction = Direction::from_str(char_str)?;
        let steps = num_str.parse::<i32>()?;
        Ok(Self { direction, steps })
    }
}

struct SafeCracker {
    start_position: i32,
    moves: Vec<Move>,
    dial_size: i32,
}

impl SafeCracker {
    pub fn new(start_position: i32, moves: Vec<Move>, dial_size: i32) -> Self {
        Self {
            start_position,
            moves,
            dial_size,
        }
    }

    pub fn from_raw_inputs(
        start_position: i32,
        moves: Vec<String>,
        dial_size: i32,
    ) -> Result<Self, ParseMoveError> {
        let parsed = moves
            .iter()
            .map(|s| s.parse::<Move>())
            .collect::<Result<Vec<Move>, ParseMoveError>>()?;
        Ok(Self {
            start_position,
            moves: parsed,
            dial_size,
        })
    }

    pub fn run(&self) -> Vec<i32> {
        let mut positions = vec![self.start_position];
        let mut current_position = self.start_position;

        for m in &self.moves {
            current_position += match m.direction {
                Direction::Left => -m.steps,
                Direction::Right => m.steps,
            };
            current_position =
                ((current_position % self.dial_size) + self.dial_size) % self.dial_size;
            positions.push(current_position);
        }
        positions
    }

    pub fn run_with_passes(&self) -> (i32, Vec<i32>) {
        let mut positions = vec![self.start_position];
        let mut current_position = self.start_position;
        let mut passes = 0;

        for m in &self.moves {
            let initial = current_position;
            current_position += match m.direction {
                Direction::Left => -m.steps,
                Direction::Right => m.steps,
            };
            let mut bias = 0;
            if current_position.is_negative() && initial != 0 {
                bias = 1
            }
            let mut new_passes = (current_position / self.dial_size).abs() + bias;
            current_position =
                ((current_position % self.dial_size) + self.dial_size) % self.dial_size;
            if current_position == 0 && new_passes > 0 {
                new_passes -= 1
            }
            passes += new_passes;
            positions.push(current_position);
        }
        (passes, positions)
    }

    pub fn count_zeros(&self) -> i32 {
        self.run().iter().filter(|&&p| p == 0).count() as i32
    }

    pub fn count_zero_incl_passes(&self) -> i32 {
        let (passes, positions) = self.run_with_passes();
        positions.iter().filter(|&&p| p == 0).count() as i32 + passes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        let moves = vec!["R50", "L50", "R100", "L1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let cracker = SafeCracker::from_raw_inputs(50, moves, 100).unwrap();
        let result = cracker.count_zeros();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_pt2() {
        let moves = vec!["R50", "L50", "L500", "L1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let cracker = SafeCracker::from_raw_inputs(50, moves, 100).unwrap();
        let result = cracker.count_zero_incl_passes();
        assert_eq!(result, 6);
    }
}
