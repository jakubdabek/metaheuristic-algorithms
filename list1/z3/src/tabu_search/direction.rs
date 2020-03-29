use crate::board::Board;
use crate::point::Point;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn move_point(self, point: Point) -> Point {
        match self {
            Direction::Up => point + Point::new(0, 1),
            Direction::Down => point - Point::new(0, 1),
            Direction::Right => point + Point::new(1, 0),
            Direction::Left => point - Point::new(1, 0),
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
