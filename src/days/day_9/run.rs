use crate::utils::advent_day::AdventDay;
use crate::utils::coordinates::{Area, Coordinate2D};
use crate::utils::load::load_from_file;
use anyhow::{anyhow, Result};
use geo::{BoundingRect, Contains, Intersects, Rect};
use geo::{Point, Polygon};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;

pub struct DayNine;

impl AdventDay for DayNine {
    fn part_1(&self) -> Result<()> {
        tracing::info!("Day 9: Part 1");
        let input = load_from_file("inputs/day_9/part1.txt")?;
        let area = get_biggest_rectangle(input)?;
        tracing::info!("Largest rectangle has area: {}", area);
        Ok(())
    }

    fn part_2(&self) -> Result<()> {
        tracing::info!("Day 9: Part 2");
        let input = load_from_file("inputs/day_9/part1.txt")?;
        let area = get_biggest_rectangle_pt2(input)?;
        tracing::info!("Largest rectangle has area: {}", area);
        Ok(())
    }
}

struct Rectangle {
    point_1: Coordinate2D,
    point_2: Coordinate2D,
}

impl Rectangle {
    fn area(&self) -> i64 {
        self.point_1.area(self.point_2)
    }

    fn corners(&self) -> Vec<Coordinate2D> {
        vec![
            self.point_1,
            self.point_2,
            Coordinate2D {
                x: self.point_1.x,
                y: self.point_2.y,
            },
            Coordinate2D {
                x: self.point_2.x,
                y: self.point_1.y,
            },
        ]
    }
}

fn get_biggest_rectangle(input: Vec<String>) -> Result<i64> {
    let coords = input
        .iter()
        .map(Coordinate2D::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let rectangle = coords
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.area(*b))
        .max()
        .ok_or(anyhow!("No max found."))?;

    Ok(rectangle)
}

fn get_biggest_rectangle_pt2(input: Vec<String>) -> Result<i64> {
    let coords = input
        .iter()
        .map(Coordinate2D::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let mut poly_points: Vec<Point<f64>> = coords
        .iter()
        .map(|c| Point::new(c.x as f64, c.y as f64))
        .collect();
    if poly_points.first() != poly_points.last() {
        poly_points.push(*poly_points.first().unwrap());
    }
    let poly = Polygon::new(poly_points.into_iter().collect(), vec![]);
    let poly_bbox = poly.bounding_rect().expect("Couldn't build bounding box");

    let mut rectangles = coords
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Rectangle {
            point_1: *a,
            point_2: *b,
        })
        .collect_vec();
    rectangles.par_sort_unstable_by(|a, b| b.area().cmp(&a.area()));
    let best_rectangle = rectangles
        .par_iter()
        .find_first(|rectangle| {
            let candidate_rect = Rect::new(
                Point::new(rectangle.point_1.x as f64, rectangle.point_1.y as f64),
                Point::new(rectangle.point_2.x as f64, rectangle.point_2.y as f64),
            );
            if !rectangle
                .corners()
                .iter()
                .all(|p| poly.intersects(&Point::new(p.x as f64, p.y as f64)))
                || !poly_bbox.contains(&candidate_rect)
            {
                return false;
            } else if poly.contains(&candidate_rect) {
                return true;
            } else {
                false
            }
        })
        .ok_or(anyhow!("No valid rectangle found."))?;

    Ok(best_rectangle.area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_biggest_rectangle_part1() {
        let test_input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
        .to_string();
        let area = get_biggest_rectangle(test_input.lines().map(String::from).collect()).unwrap();
        assert_eq!(area, 50);
    }

    #[test]
    fn test_get_biggest_rectangle_part2() {
        let test_input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
        .to_string();
        let area =
            get_biggest_rectangle_pt2(test_input.lines().map(String::from).collect()).unwrap();
        assert_eq!(area, 24);
    }
}
