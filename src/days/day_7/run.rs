use crate::utils::advent_day::AdventDay;
use crate::utils::load::load_from_file;
use crate::utils::sets::inplace_intersection;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub struct DaySeven;

impl AdventDay for DaySeven {
    fn part_1(&self) -> Result<()> {
        tracing::info!("Day 7: Part 1");
        let input = load_from_file("inputs/day_7/part1.txt")?;
        let count = count_beam_splits(input)?;
        tracing::info!("Beam is split {} times", count);
        Ok(())
    }

    fn part_2(&self) -> Result<()> {
        tracing::info!("Day 7: Part 2");
        let input = load_from_file("inputs/day_7/part1.txt")?;
        let count = count_total_timelines(input)?;
        tracing::info!("Found {} total paths", count);
        Ok(())
    }
}

fn count_beam_splits(input: Vec<String>) -> Result<i32> {
    let mut count = 0;
    let mut splitter_indexes = vec![];
    let mut beam_indexes: Vec<usize> = vec![];
    for line in input.iter() {
        for (i, char) in line.chars().enumerate() {
            if char == '^' {
                splitter_indexes.push(i);
            }
            if char == 'S' {
                beam_indexes.push(i);
            }
        }
        let overlap: HashSet<usize> = inplace_intersection(
            &mut HashSet::<usize>::from_iter(beam_indexes.iter().copied()),
            &mut HashSet::from_iter(splitter_indexes.iter().copied()),
        );
        count += overlap.len();
        beam_indexes = get_new_beams(beam_indexes, overlap.into_iter().collect(), line.len());
        splitter_indexes.clear();
    }

    Ok(count as i32)
}

pub fn get_new_beams(mut current: Vec<usize>, overlap: Vec<usize>, max: usize) -> Vec<usize> {
    for i in overlap {
        current.retain(|x| x != &i);
        if i != 0 {
            current.push(i - 1);
        }
        if i != max {
            current.push(i + 1);
        }
    }

    current
}

fn count_total_timelines(input: Vec<String>) -> Result<u64> {
    let mut timeline_counts: HashMap<usize, u64> = HashMap::new();

    for (i, char) in input[0].chars().enumerate() {
        if char == 'S' {
            timeline_counts.insert(i, 1);
        }
    }

    for line in input.iter().skip(1) {
        let mut next_counts: HashMap<usize, u64> = HashMap::new();

        for (&col_idx, &count) in &timeline_counts {
            let char = line.chars().nth(col_idx).unwrap_or('.');

            match char {
                '^' => {
                    *next_counts.entry(col_idx - 1).or_insert(0) += count;
                    *next_counts.entry(col_idx + 1).or_insert(0) += count;
                }
                _ => {
                    *next_counts.entry(col_idx).or_insert(0) += count;
                }
            }
        }
        timeline_counts = next_counts;
    }

    Ok(timeline_counts.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_beams_part1() {
        let test_input = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
        .to_string();
        let total = count_beam_splits(test_input.lines().map(String::from).collect()).unwrap();
        assert_eq!(total, 21);
    }

    #[test]
    fn test_count_beams_pt2() {
        let test_input = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
        .to_string();
        let total = count_total_timelines(test_input.lines().map(String::from).collect()).unwrap();
        assert_eq!(total, 40);
    }
}
