use crate::point::Point;
use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use std::fmt;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub const DIRECTIONS: &[Direction] = &[
    Direction::Left,
    Direction::Up,
    Direction::Right,
    Direction::Down,
];

impl Direction {
    pub fn inverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    pub fn move_point(self, point: Point) -> Point {
        match self {
            Direction::Up => point + Point::new(0, 1),
            Direction::Down => point - Point::new(0, 1),
            Direction::Right => point + Point::new(1, 0),
            Direction::Left => point - Point::new(1, 0),
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        use Direction::*;
        match rng.gen_range(0, 5) {
            0 => Up,
            1 => Down,
            2 => Right,
            3 => Left,
            _ => unreachable!("gen_range produced value out of range"),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "U"),
            Direction::Down => write!(f, "D"),
            Direction::Right => write!(f, "R"),
            Direction::Left => write!(f, "L"),
        }
    }
}
