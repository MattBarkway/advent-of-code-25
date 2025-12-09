use std::ops::{Add, Sub};
use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[error("Invalid coordinate. should be a string of 3 numbers separated by commas: 'x,y,z'")]
pub struct ParseCoordinateError;

fn get_coord_values(
    raw_coord: &str,
    expected_len: usize,
) -> Result<Vec<i64>, ParseCoordinateError> {
    let split: Vec<i64> = raw_coord
        .split(',')
        .map(|i| Ok(i.parse::<i64>().map_err(|_| ParseCoordinateError))?)
        .collect::<anyhow::Result<Vec<_>, _>>()?;
    if split.len() != expected_len {
        return Err(ParseCoordinateError);
    }
    Ok(split)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl TryFrom<&String> for Coordinate3D {
    type Error = ParseCoordinateError;

    fn try_from(value: &String) -> std::result::Result<Self, Self::Error> {
        let split = get_coord_values(value, 3)?;
        Ok(Self {
            x: split[0],
            y: split[1],
            z: split[2],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate2D {
    pub x: i64,
    pub y: i64,
}

impl TryFrom<&String> for Coordinate2D {
    type Error = ParseCoordinateError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let split = get_coord_values(value, 2)?;
        Ok(Self {
            x: split[0],
            y: split[1],
        })
    }
}

impl Add for Coordinate2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coordinate2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub trait Distance {
    fn distance_to(&self, other: Self) -> f64;
}

impl Distance for Coordinate3D {
    fn distance_to(&self, other: Coordinate3D) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).abs()
            as f64)
            .sqrt()
    }
}

impl Distance for Coordinate2D {
    fn distance_to(&self, other: Coordinate2D) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)).abs() as f64).sqrt()
    }
}

pub trait Area {
    fn area(&self, other: Self) -> i64;
}

impl Area for Coordinate2D {
    fn area(&self, other: Self) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}
