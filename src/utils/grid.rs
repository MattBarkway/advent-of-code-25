use itertools::Itertools;
use std::fmt;
use std::fmt::{Debug, Display};
use std::slice::Iter;

use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[error("Invalid Grid. should be a rectangular grid of values")]
pub struct ParseGridError;

#[derive(Clone)]
pub struct Grid<T: Clone> {
    pub width: usize,
    pub values: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, values: Vec<Vec<T>>) -> Self {
        Self { width, values }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.values[y][x].clone()
    }

    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.values.iter()
    }

    fn neighbour_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        (-1..2)
            .cartesian_product(-1..2)
            .filter(|(x, y)| !(x == &0 && y == &0))
            .filter_map(|(dx, dy)| {
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                if nx < self.width && ny < self.values.len() {
                    Some((nx, ny))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn neighbours(&self, x: usize, y: usize) -> Vec<T> {
        self.neighbour_coords(x, y)
            .iter()
            .map(|&(x, y)| self.get(x, y))
            .collect()
    }

    pub(crate) fn transpose(&self, default: T) -> Grid<T> {
        if self.values.is_empty() || self.values[0].is_empty() {
            return Grid::new(self.width, self.values.clone());
        }
        let mut iters: Vec<_> = self
            .values
            .clone()
            .into_iter()
            .map(|v| v.into_iter())
            .collect();
        let num_rows = iters[0].len();
        let transposed: Vec<Vec<T>> = (0..num_rows)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|it| it.next().unwrap_or(default.clone()))
                    .collect::<Vec<T>>()
            })
            .collect();
        Grid::new(transposed[0].len(), transposed)
    }
}

impl<T: Clone + Display + Debug> Display for Grid<T> {
    // T must also implement Display
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid (Width: {})", self.width)?;

        for row in &self.values {
            write!(f, "[")?;
            let elements: Vec<String> = row.iter().map(|item| item.to_string()).collect();
            write!(f, "{}", elements.join(" "))?;
            writeln!(f, "]")?;
        }
        Ok(())
    }
}
