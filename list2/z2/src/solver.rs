use itertools::Itertools;
use ndarray::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::{BTreeSet, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::io::BufRead;
use std::time::{Duration, Instant};

type Value = u8;
type ValueMatrix = Array2<Value>;
type Distance = f64;

#[derive(Debug, Clone)]
pub struct Solver {
    values: ValueMatrix,
    block_size: usize,
    time_limit: Duration,
}

pub struct Solution {
    pub matrix: ValueMatrix,
    pub distance: Distance,
}

impl Solver {
    pub fn new(values: ValueMatrix, block_size: usize, time_limit: Duration) -> Self {
        Self {
            values,
            block_size,
            time_limit,
        }
    }

    pub fn search(&self) -> Solution {
        todo!()
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum SolverCreationError {
    InvalidHeader,
    InvalidLine,
    NotEnoughLines,
    TooManyLines,
    IOError(String),
}

impl fmt::Display for SolverCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolverCreationError::InvalidHeader => write!(f, "Invalid header (first line)"),
            SolverCreationError::InvalidLine => write!(f, "Invalid data in a line"),
            SolverCreationError::NotEnoughLines => write!(f, "Not enough lines"),
            SolverCreationError::TooManyLines => write!(f, "Too many lines"),
            SolverCreationError::IOError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for SolverCreationError {}

impl From<std::io::Error> for SolverCreationError {
    fn from(err: std::io::Error) -> Self {
        SolverCreationError::IOError(err.to_string())
    }
}

impl Solver {
    pub fn try_from_read<R: BufRead>(reader: R) -> Result<Solver, SolverCreationError> {
        use SolverCreationError::*;

        let mut lines = reader.lines();
        let header = lines
            .next()
            .ok_or(NotEnoughLines)??
            .split_ascii_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| InvalidHeader)?;

        let (time, n, m, block_size) = match *header.as_slice() {
            [time, n, m, block_size]
                if time > 0 && block_size > 0 && block_size <= n && block_size <= m =>
            {
                (time, n, m, block_size)
            }
            _ => return Err(InvalidHeader),
        };

        let mut values = ValueMatrix::zeros((n, m));

        for zipped in values.outer_iter_mut().zip_longest(lines) {
            use itertools::EitherOrBoth::*;
            let (mut row, line) = match zipped {
                Both(row, line) => (row, line),
                Left(_) => return Err(NotEnoughLines),
                Right(_) => return Err(TooManyLines),
            };

            let line = line?;
            let values = line.split_ascii_whitespace().map(str::parse::<Value>);

            row.iter_mut()
                .zip_longest(values)
                .try_for_each(|zipped| match zipped {
                    Both(elem, value) => Ok(*elem = value.map_err(|_| InvalidLine)?),
                    Left(_) | Right(_) => Err(InvalidLine),
                })?;
        }

        Ok(Solver::new(
            values,
            block_size,
            Duration::from_secs(time.try_into().unwrap()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod creation {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn create_ok1() -> Result<(), SolverCreationError> {
            let input = "1 1 1 1\n0";
            Solver::try_from_read(Cursor::new(input)).map(|_| ())
        }

        #[test]
        fn create_ok2() -> Result<(), SolverCreationError> {
            let input = "1 2 2 1\n0 1\n1 0";
            Solver::try_from_read(Cursor::new(input)).map(|_| ())
        }

        #[test]
        fn create_err_parse_time() {
            let input = "a 2 2 1\n0 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_parse_n() {
            let input = "1 a 2 1\n0 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_parse_m() {
            let input = "1 2 a 1\n0 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_0_seconds() {
            let input = "0 2 2 1\n0 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_not_enough_lines() {
            let input = "1 2 2 1\n0 1";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::NotEnoughLines))
        }

        #[test]
        fn create_err_too_many_lines() {
            let input = "1 2 2 1\n0 1\n1 0\n1 1";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::TooManyLines))
        }

        #[test]
        fn create_err_line_parse() {
            let input = "1 2 2 1\na 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidLine))
        }

        #[test]
        fn create_err_line_too_many() {
            let input = "1 2 2 1\n0 1 2\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidLine))
        }

        #[test]
        fn create_err_line_not_enough() {
            let input = "1 2 2 1\n0\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidLine))
        }
    }
}
