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

    pub fn remove_redundancies(moves: &mut Vec<Direction>) {
        let mut i = 0;
        while i + 2 < moves.len() {
            if moves[i].inverse() == moves[i + 1] {
                drop(moves.drain(i..=(i + 1)));
                if i > 0 {
                    i -= 1;
                }
            } else if moves[i].inverse() == moves[i + 2] {
                moves.swap(i + 1, i + 2);
                drop(moves.drain(i..=(i + 1)));
                if i > 0 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        use Direction::*;
        match rng.gen_range(0, 4) {
            0 => Up,
            1 => Down,
            2 => Right,
            3 => Left,
            _ => unreachable!("gen_range produced value out of range"),
        }
    }
}

impl Direction {
    pub fn parse(c: u8) -> Option<Self> {
        match c {
            b'U' => Some(Self::Up),
            b'D' => Some(Self::Down),
            b'R' => Some(Self::Right),
            b'L' => Some(Self::Left),
            _ => None,
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
