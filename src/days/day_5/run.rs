use crate::utils::advent_day::AdventDay;
use crate::utils::load::raw_load_from_file;
use anyhow::{anyhow, Error, Result};

pub struct DayFive;

impl AdventDay for DayFive {
    fn part_1(&self) -> Result<()> {
        tracing::info!("Day 5: Part 1");
        let input = raw_load_from_file("inputs/day_5/part1.txt")?;
        let count = count_fresh(&input)?;
        tracing::info!("Found {} fresh ingredients", count);
        Ok(())
    }

    fn part_2(&self) -> Result<()> {
        tracing::info!("Day 5: Part 2");
        let input = raw_load_from_file("inputs/day_5/part1.txt")?;
        let count = count_all_possible_fresh(&input)?;
        tracing::info!("Found {} valid paper", count);
        Ok(())
    }
}

struct RangeSet {
    ranges: Vec<(i64, i64)>,
}

impl TryFrom<&str> for RangeSet {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(Self {
            ranges: value
                .lines()
                .map(|line| {
                    let (min_str, max_str) =
                        line.split_once("-").ok_or(anyhow!("Missing delimiter"))?;
                    Ok((min_str.parse::<i64>()?, max_str.parse::<i64>()?))
                })
                .collect::<Result<_>>()?,
        })
    }
}

impl RangeSet {
    fn condense(&self) -> Vec<(i64, i64)> {
        let mut ranges = self.ranges.clone();
        ranges.sort_by(|(min_a, _), (min_b, _)| min_a.cmp(min_b));
        ranges
            .into_iter()
            .fold(Vec::new(), |mut acc, (c_min, c_max)| {
                if let Some(e) = acc.last_mut() {
                    let (_, e_max) = *e;
                    if c_min <= e_max {
                        e.1 = e_max.max(c_max);
                    } else {
                        acc.push((c_min, c_max));
                    }
                } else {
                    acc.push((c_min, c_max));
                }
                acc
            })
    }
}

fn split_ingredients(raw_ingredients: &str) -> Result<(&str, &str)> {
    raw_ingredients
        .split_once("\n\n")
        .ok_or(anyhow!("Missing ingredient separator"))
}

fn parse_available(raw_available: &str) -> Result<Vec<i64>> {
    raw_available
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .map_err(|e| anyhow!("Invalid ingredient"))
        })
        .collect::<Result<_>>()
}

fn count_overlapping(ranges: Vec<(i64, i64)>, points: Vec<i64>) -> usize {
    points
        .iter()
        .filter_map(|ingredient| {
            ranges
                .iter()
                .find(|range| ingredient >= &range.0 && ingredient <= &range.1)
        })
        .count()
}

fn count_fresh(input: &str) -> Result<usize> {
    let (raw_fresh, raw_available) = split_ingredients(input)?;
    let fresh = RangeSet::try_from(raw_fresh)?.condense();
    let count = count_overlapping(fresh, parse_available(raw_available)?);

    Ok(count)
}

fn count_all_possible_fresh(input: &str) -> Result<i64> {
    tracing::info!("Counting ingredients");
    let (raw_fresh, _) = split_ingredients(input)?;
    let fresh = RangeSet::try_from(raw_fresh)?.condense();
    Ok(fresh
        .into_iter()
        .fold(0, |acc, (min, max)| acc + max - min + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_fresh() {
        let test_input = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let count = count_fresh(test_input).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_all_possible_fresh() {
        let test_input = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let count = count_all_possible_fresh(test_input).unwrap();
        assert_eq!(count, 14);
    }
}
