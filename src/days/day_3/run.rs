use crate::days::day_3::errors::ParseBankError;
use crate::utils::advent_day::AdventDay;
use crate::utils::load::load_from_file;
use anyhow::{Error, Result};
use std::cmp::{max, Ordering};

pub struct DayThree;

impl AdventDay for DayThree {
    fn part_1(&self) -> Result<()> {
        tracing::info!("Day 3: Part 1");
        let banks = load_from_file("inputs/day_3/part1.txt")?;
        let values: Result<Vec<u64>> = banks
            .iter()
            .map(|bank| maximise_joltage_n_times(bank, 2))
            .collect();
        let sum: u64 = values?.iter().sum();

        tracing::info!("The sum of max joltages is {}", sum);
        Ok(())
    }

    fn part_2(&self) -> Result<()> {
        tracing::info!("Day 3: Part 2");

        let banks = load_from_file("inputs/day_3/part1.txt")?;
        let values: Result<Vec<u64>> = banks
            .iter()
            .map(|bank| maximise_joltage_n_times(bank, 12))
            .collect();
        let sum: u64 = values?.iter().sum();

        tracing::info!("The sum of max joltages is {}", sum);
        Ok(())
    }
}

fn get_max_char(val: &str) -> Result<(u32, usize)>
{
    val.chars()
        .enumerate()
        .filter_map(|(idx, c)| {
            c.to_digit(10).map(|d| (d, idx))
        })
        .max_by(|(d_a, idx_a), (d_b, idx_b)| match d_a.cmp(d_b) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => idx_b.cmp(idx_a),
        })
        .ok_or(Error::from(ParseBankError))
}

pub fn maximise_joltage_n_times(bank: &str, n: usize) -> Result<u64> {
    let mut acc = vec![];
    let mut prev_idx = 0;
    for i in 0..n {
        let slice = &bank[prev_idx..bank.len() - (max(n - i - 1, 0))];
        let (max_joltage, idx) = get_max_char(slice)?;
        acc.push(max_joltage);
        prev_idx = prev_idx + idx + 1;
    }
    let result_string: String = acc.iter().map(|d| d.to_string()).collect();
    if result_string.is_empty() {
        return Err(ParseBankError.into());
    }
    Ok(result_string.parse::<u64>().map_err(|_| ParseBankError)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximise_joltage() {
        let joltage = maximise_joltage_n_times("811111111111119", 2).unwrap();
        assert_eq!(joltage, 89);

        let joltage = maximise_joltage_n_times("11", 2).unwrap();
        assert_eq!(joltage, 11);

        let joltage = maximise_joltage_n_times("9111111118", 2).unwrap();
        assert_eq!(joltage, 98);

        let joltage = maximise_joltage_n_times("123456789", 2).unwrap();
        assert_eq!(joltage, 89);

        let joltage = maximise_joltage_n_times("1234567899", 2).unwrap();
        assert_eq!(joltage, 99);
    }

    #[test]
    fn test_maximise_joltage_n_times() {
        let joltage = maximise_joltage_n_times("987654321111111", 12).unwrap();
        assert_eq!(joltage, 987654321111);

        let joltage = maximise_joltage_n_times("811111111111119", 12).unwrap();
        assert_eq!(joltage, 811111111119);

        let joltage = maximise_joltage_n_times("234234234234278", 12).unwrap();
        assert_eq!(joltage, 434234234278);

        let joltage = maximise_joltage_n_times("818181911112111", 12).unwrap();
        assert_eq!(joltage, 888911112111);
    }
}
