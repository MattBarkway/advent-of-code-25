use crate::days::day_4::errors::ParseGridError;
use crate::utils::advent_day::AdventDay;
use crate::utils::load::raw_load_from_file;
use anyhow::Result;
use itertools::Itertools;
use std::slice::Iter;
use std::str::FromStr;

pub struct DayFour;

impl AdventDay for DayFour {
    fn part_1(&self) -> Result<()> {
        tracing::info!("Day 4: Part 1");
        let input = raw_load_from_file("inputs/day_4/part1.txt")?;
        let grid = Grid::from_str(&input)?;
        let count = find_paper(grid)?;

        tracing::info!("Found {} valid paper", count);
        Ok(())
    }

    fn part_2(&self) -> Result<()> {
        tracing::info!("Day 4: Part 2");
        let input = raw_load_from_file("inputs/day_4/part1.txt")?;
        let grid = Grid::from_str(&input)?;
        let count = find_and_remove_paper(grid)?;

        tracing::info!("Found {} valid paper", count);
        Ok(())
    }
}

#[derive(Clone)]
struct Grid {
    width: usize,
    plan: Vec<Vec<char>>,
}

impl Grid {
    pub fn get(&self, x: usize, y: usize) -> char {
        self.plan[y][x]
    }

    pub fn iter(&self) -> Iter<'_, Vec<char>> {
        self.plan.iter()
    }

    pub fn remove(&mut self, x: usize, y: usize) {
        self.plan[y][x] = 'x'
    }

    fn neighbour_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        (-1..2)
            .cartesian_product(-1..2)
            .filter(|(x, y)| !(x == &0 && y == &0))
            .filter_map(|(dx, dy)| {
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                if nx < self.width && ny < self.plan.len() {
                    Some((nx, ny))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn neighbours(&self, x: usize, y: usize) -> Vec<char> {
        self.neighbour_coords(x, y)
            .iter()
            .map(|&(x, y)| self.get(x, y))
            .collect()
    }
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let plan: Vec<Vec<char>> = s
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(String::from)
            .map(|s| s.chars().collect())
            .collect();
        let width = plan[0].len();
        Ok(Grid { plan, width })
    }
}

fn count_of_char(chars: Vec<char>, c: char) -> usize {
    chars.iter().filter(|&&x| x == c).count()
}

fn find_paper(plan: Grid) -> Result<i32> {
    let mut valid_count = 0;

    for (y_idx, row) in plan.iter().enumerate() {
        for (x_idx, symbol) in row.iter().enumerate() {
            if symbol == &'@' {
                let neighbours = count_of_char(plan.neighbours(x_idx, y_idx), '@');
                if neighbours < 4 {
                    valid_count += 1
                }
            }
        }
    }
    Ok(valid_count)
}

fn find_and_remove_paper(mut plan: Grid) -> Result<i32> {
    let mut prev_count: i32 = 0;
    let mut valid_count = 0;
    let mut flag = true;

    while prev_count != valid_count || flag {
        flag = false;
        prev_count = valid_count;
        for (y_idx, row) in plan.clone().iter().enumerate() {
            for (x_idx, symbol) in row.iter().enumerate() {
                if symbol == &'@' {
                    let neighbours = count_of_char(plan.neighbours(x_idx, y_idx), '@');
                    if neighbours < 4 {
                        valid_count += 1;
                        plan.remove(x_idx, y_idx);
                    }
                }
            }
        }
    }
    Ok(valid_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_paper() {
        let test_input = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = Grid::from_str(test_input).unwrap();
        let count = find_paper(grid).unwrap();
        assert_eq!(count, 13);
    }

    #[test]
    fn test_find_and_remove_paper() {
        let test_input = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = Grid::from_str(test_input).unwrap();
        let count = find_and_remove_paper(grid).unwrap();
        assert_eq!(count, 43);
    }
}
