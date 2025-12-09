use crate::utils::advent_day::AdventDay;
use crate::utils::coordinates::{Coordinate3D, Distance};
use crate::utils::load::load_from_file;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Debug;

pub struct DayEight;

impl AdventDay for DayEight {
    fn part_1(&self) -> Result<()> {
        tracing::info!("Day 8: Part 1");
        let input = load_from_file("inputs/day_8/part1.txt")?;
        let product = build_circuits(input, 1000)?;
        tracing::info!("Circuit product is: {}", product);
        Ok(())
    }

    fn part_2(&self) -> Result<()> {
        tracing::info!("Day 8: Part 2");
        let input = load_from_file("inputs/day_8/part1.txt")?;
        let count = build_circuits_pt2(input)?;
        tracing::info!("Product is {}", count);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Circuit {
    coords: HashSet<Coordinate3D>,
}

impl Circuit {
    fn includes(&self, coord: Coordinate3D) -> bool {
        self.coords.contains(&coord)
    }

    fn add(&mut self, coord: Coordinate3D) {
        self.coords.insert(coord);
    }

    fn combine(&mut self, other: &Self) {
        self.coords.extend(other.coords.iter().cloned());
    }
}

fn get_sorted_pairs(coords: &[Coordinate3D]) -> Vec<(f64, Coordinate3D, Coordinate3D)> {
    let mut sorted_pairs = coords
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            if a == b {
                None
            } else {
                Some((a.distance_to(*b), *a, *b))
            }
        })
        .collect::<Vec<_>>();

    sorted_pairs
        .sort_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal));
    sorted_pairs
}

fn build_circuits(input: Vec<String>, n: usize) -> Result<i64> {
    let mut circuits: Vec<Circuit> = vec![];
    let coords = input
        .iter()
        .map(Coordinate3D::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let sorted_pairs = get_sorted_pairs(&coords);
    for (_, a, b) in sorted_pairs.iter().take(n) {
        let linkable_indices = circuits
            .iter()
            .enumerate()
            .filter(|(_, c)| c.includes(*a) || c.includes(*b))
            .map(|(i, _)| i)
            .collect_vec();

        if linkable_indices.is_empty() {
            circuits.push(Circuit {
                coords: HashSet::from([*a, *b]),
            });
        } else if linkable_indices.len() == 1 {
            let idx = linkable_indices[0];
            circuits[idx].add(*a);
            circuits[idx].add(*b);
        } else if linkable_indices.len() == 2 {
            let circuit2 = circuits[linkable_indices[1]].clone();
            circuits[linkable_indices[0]].combine(&circuit2);
            circuits.remove(linkable_indices[1]);
        } else {
            Err(anyhow!("Invalid state"))?
        }
    }
    circuits.sort_by(|a, b| b.coords.len().cmp(&a.coords.len()));
    Ok(circuits
        .iter()
        .take(3)
        .map(|coord| coord.coords.len() as i64)
        .product())
}

fn build_circuits_pt2(input: Vec<String>) -> Result<i64> {
    let coords = input
        .iter()
        .map(Coordinate3D::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let mut circuits: Vec<Circuit> = coords
        .iter()
        .map(|c| Circuit {
            coords: HashSet::from([*c]),
        })
        .collect();
    let sorted_pairs = get_sorted_pairs(&coords);
    for (_, a, b) in sorted_pairs {
        let linkable_indices = circuits
            .iter()
            .enumerate()
            .filter(|(_, c)| c.includes(a) || c.includes(b))
            .map(|(i, _)| i)
            .collect_vec();

        if linkable_indices.is_empty() {
            circuits.push(Circuit {
                coords: HashSet::from([a, b]),
            });
        } else if linkable_indices.len() == 1 {
            let idx = linkable_indices[0];
            circuits[idx].add(a);
            circuits[idx].add(b);
        } else if linkable_indices.len() == 2 {
            let circuit2 = circuits[linkable_indices[1]].clone();
            circuits[linkable_indices[0]].combine(&circuit2);
            circuits.remove(linkable_indices[1]);
            if circuits.len() == 1 {
                return Ok(a.x * b.x);
            }
        } else {
            Err(anyhow!("Invalid state"))?
        }
    }
    Err(anyhow!("no solution found"))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_circuits_part1() {
        let test_input = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
        .to_string();
        let total = build_circuits(test_input.lines().map(String::from).collect(), 10).unwrap();
        assert_eq!(total, 40);
    }

    #[test]
    fn test_build_circuits_part2() {
        let test_input = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
        .to_string();
        let total = build_circuits_pt2(test_input.lines().map(String::from).collect()).unwrap();
        assert_eq!(total, 25272);
    }
}
