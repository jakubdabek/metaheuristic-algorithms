use crate::point::Point;
use crate::tabu_search::direction::{Direction, DIRECTIONS};
use itertools::{EitherOrBoth, Itertools};
use ndarray::prelude::*;
use ndarray::IntoDimension;
use std::fmt;
use std::io::BufRead;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub enum Field {
    Empty,
    Wall,
    Exit,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub fields: Array2<Field>,
    pub agent_position: Point,
}

impl Board {
    #[inline]
    pub fn in_bounds(&self, point: Point) -> bool {
        let (w, h) = self.fields.dim();
        point.x < (w - 1) as _ && point.y < (h - 1) as _ && point.x > 0 && point.y > 0
    }

    #[inline]
    pub fn is_next_to_edge(&self, point: Point) -> bool {
        let (w, h) = self.fields.dim();
        point.x == 1 || point.x == (w - 2) as _ || point.y == 1 || point.y == (h - 2) as _
    }

    #[inline]
    pub fn is_valid_position(&self, point: Point) -> bool {
        self.in_bounds(point) && !matches!(self.fields[point.into_dimension()], Field::Wall)
    }

    #[inline]
    pub fn is_exit(&self, point: Point) -> bool {
        self.fields
            .get(point.into_dimension())
            .map_or(false, |f| matches!(f, Field::Exit))
    }

    pub fn move_into_exit(&self, point: Point) -> Option<(Direction, Point)> {
        debug_assert!(self.is_valid_position(point));
        for &dir in DIRECTIONS {
            let point = dir.move_point(point);
            if matches!(self.fields[point.into_dimension()], Field::Exit) {
                return Some((dir, point));
            }
        }

        None
    }

    pub fn adjacent_positions(&self, point: Point) -> impl Iterator<Item = (Direction, Point)> {
        assert!(
            self.in_bounds(point),
            "only points inbounds have adjacent ones"
        );
        DIRECTIONS.iter().map(move |&x| (x, x.move_point(point)))
    }

    pub fn adjacent(&self, point: Point) -> impl Iterator<Item = (Direction, Point, Field)> + '_ {
        self.adjacent_positions(point)
            .filter_map(move |(d, p)| self.fields.get(p.into_dimension()).map(|&f| (d, p, f)))
    }

    pub fn adjacent_in_bounds(
        &self,
        point: Point,
    ) -> impl Iterator<Item = (Direction, Point, Field)> + '_ {
        debug_assert!(
            self.in_bounds(point),
            "adjacent_in_bounds called with out of bounds point"
        );

        self.adjacent_positions(point)
            .map(move |(d, p)| (d, p, self.fields[p.into_dimension()]))
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum BoardCreationError {
    InvalidHeader,
    InvalidLine,
    NotEnoughLines,
    InvalidGoal,
    InvalidAgent,
    IOError(String),
}

impl fmt::Display for BoardCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardCreationError::InvalidHeader => write!(f, "Invalid header (first line)"),
            BoardCreationError::InvalidLine => write!(f, "Invalid data in a line"),
            BoardCreationError::NotEnoughLines => write!(f, "Not enough lines"),
            BoardCreationError::InvalidGoal => write!(f, "Invalid goal"),
            BoardCreationError::InvalidAgent => write!(f, "Invalid agent"),
            BoardCreationError::IOError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for BoardCreationError {}

impl From<std::io::Error> for BoardCreationError {
    fn from(err: std::io::Error) -> Self {
        BoardCreationError::IOError(err.to_string())
    }
}

impl Board {
    pub fn try_from_read<R: BufRead>(reader: R) -> Result<(Board, Duration), BoardCreationError> {
        use BoardCreationError::*;

        let mut lines = reader.lines();
        let header = lines
            .next()
            .ok_or(NotEnoughLines)??
            .split_ascii_whitespace()
            .map(str::parse::<u64>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| InvalidHeader)?;

        let (time, n, m) = match *header.as_slice() {
            [time, n, m] if time > 0 => (time, n, m),
            _ => return Err(InvalidHeader),
        };

        let mut agent = None;

        let on_horizontal_edge = |i| i == n - 1 || i == 0;
        let on_vertical_edge = |j| j == 0 || j == m - 1;
        let on_edge = |i, j| on_horizontal_edge(i) || on_vertical_edge(j);
        let in_corner = |i, j| on_horizontal_edge(i) && on_vertical_edge(j);

        let mut fields = Array2::from_elem((n as _, m as _), Field::Empty);

        for it in fields.outer_iter_mut().enumerate().rev().zip_longest(lines) {
            let (i, mut row, line) = match it {
                EitherOrBoth::Both((i, row), line) => {
                    let line = line?;
                    if line.as_bytes().len() != m as _ {
                        return Err(InvalidLine);
                    }
                    (i as _, row, line)
                }
                EitherOrBoth::Left(_) => return Err(NotEnoughLines),
                EitherOrBoth::Right(_) => break, // too many lines
            };

            let line = line.as_bytes();

            for ((c, j), field) in line.iter().zip(0..).zip(row.iter_mut()) {
                use Field::*;
                match c {
                    b'8' if !on_edge(i, j) => return Err(InvalidGoal),
                    b'8' if in_corner(i, j) => return Err(InvalidGoal),
                    b'8' => *field = Exit,

                    b'5' if on_edge(i, j) => return Err(InvalidAgent),
                    b'5' if agent.is_some() => return Err(InvalidAgent),
                    b'5' => agent = Some(Point::new(j, i)),

                    b'1' => *field = Wall,
                    b'0' if !on_edge(i, j) => (), // *field = Empty,

                    _ => return Err(InvalidLine),
                }
            }
        }

        let agent_position = agent.ok_or(InvalidAgent)?;

        Ok((
            Board {
                fields,
                agent_position,
            },
            Duration::from_secs(time),
        ))
    }
}
