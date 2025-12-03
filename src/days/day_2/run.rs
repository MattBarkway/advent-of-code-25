use crate::days::day_2::errors::ParseProductError;
use crate::days::day_2::product_range::ProductRange;
use crate::utils::advent_day::AdventDay;
use crate::utils::load::load_from_file;
use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;

pub struct DayTwo;

pub fn get_products() -> Result<Vec<ProductRange>> {
    Ok(load_from_file("inputs/day_2/part1.txt")?[0]
        .split(',')
        .map(ProductRange::from_str)
        .collect::<Result<Vec<ProductRange>, ParseProductError>>()?)
}

impl AdventDay for DayTwo {
    fn part_1(&self) -> Result<()> {
        tracing::info!("Day 2: Part 1");
        let sum: u64 = get_products()?
            .iter()
            .flat_map(|r| r.get_repeating_twice())
            .sum();
        tracing::info!("The sum of invalid products is {}", sum);
        Ok(())
    }

    fn part_2(&self) -> Result<()> {
        tracing::info!("Day 2: Part 2");
        let sum: u64 = get_products()?
            .iter()
            .flat_map(|r| r.get_repeating())
            .collect::<HashSet<_>>()
            .iter()
            .sum();
        tracing::info!("The sum of invalid products is {}", sum);
        Ok(())
    }
}
