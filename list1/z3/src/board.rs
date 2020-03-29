use crate::point::Point;
use std::convert::TryInto;
use std::fmt;
use std::io::BufRead;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    bounds: Point,
    agent_position: Point,
    goal_position: Point,
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
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| InvalidHeader)?;

        let (time, n, m) = match *header.as_slice() {
            [time, n, m] if time > 0 => (time, n, m),
            _ => return Err(InvalidHeader),
        };

        let mut agent = None;
        let mut goal = None;

        let on_horizontal_edge = |i| i == n - 1 || i == 0;
        let on_vertical_edge = |j| j == 0 || j == m - 1;
        let on_edge = |i, j| on_horizontal_edge(i) || on_vertical_edge(j);
        let in_corner = |i, j| on_horizontal_edge(i) && on_vertical_edge(j);

        for i in (0..n).rev() {
            let line: String = lines.next().ok_or(NotEnoughLines)??;
            let line = line.as_bytes();
            if line.len() != m {
                return Err(InvalidLine);
            }

            for (j, c) in line.iter().enumerate() {
                match c {
                    b'8' if !on_edge(i, j) => return Err(InvalidGoal),
                    b'8' if in_corner(i, j) => return Err(InvalidGoal),
                    b'8' if goal.is_some() => return Err(InvalidGoal),
                    b'8' => goal = Some(Point::new(j, i)),

                    b'5' if on_edge(i, j) => return Err(InvalidAgent),
                    b'5' if agent.is_some() => return Err(InvalidAgent),
                    b'5' => agent = Some(Point::new(j, i)),

                    b'1' if on_edge(i, j) => (),
                    b'0' if !on_edge(i, j) => (),

                    _ => return Err(InvalidLine),
                }
            }
        }

        let agent = agent.ok_or(InvalidAgent)?;
        let goal = goal.ok_or(InvalidGoal)?;

        Ok((
            Board {
                bounds: Point::new(m - 1, n - 1),
                agent_position: agent,
                goal_position: goal,
            },
            Duration::from_secs(time.try_into().unwrap()),
        ))
    }
}
