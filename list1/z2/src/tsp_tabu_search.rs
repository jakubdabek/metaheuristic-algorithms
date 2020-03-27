use itertools::Itertools;
use ndarray::prelude::*;
use std::convert::TryInto;
use std::fmt;
use std::io::BufRead;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct Solver {
    distances: Array2<u32>,
    time_limit: Duration,
}

impl Solver {
    pub fn new(distances: Array2<u32>, time_limit: Duration) -> Self {
        Self {
            distances,
            time_limit,
        }
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

        let (time, n) = match *header.as_slice() {
            [time, n] if time > 0 => (time, n),
            _ => return Err(InvalidHeader),
        };

        let mut matrix = Array2::zeros((n, n));

        for zipped in matrix.outer_iter_mut().zip_longest(lines) {
            use itertools::EitherOrBoth::*;
            let (mut row, line) = match zipped {
                Both(row, line) => (row, line),
                Left(_) => return Err(NotEnoughLines),
                Right(_) => return Err(TooManyLines),
            };

            let line = line?;
            let values = line.split_ascii_whitespace().map(str::parse::<u32>);

            row.iter_mut()
                .zip_longest(values)
                .try_for_each(|zipped| match zipped {
                    Both(elem, value) => Ok(*elem = value.map_err(|_| InvalidLine)?),
                    Left(_) | Right(_) => Err(InvalidLine),
                })?;
        }

        Ok(Solver::new(
            matrix,
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
            let input = "1 1\n0";
            Solver::try_from_read(Cursor::new(input)).map(|_| ())
        }

        #[test]
        fn create_ok2() -> Result<(), SolverCreationError> {
            let input = "1 2\n0 1\n1 0";
            Solver::try_from_read(Cursor::new(input)).map(|_| ())
        }

        #[test]
        fn create_err_parse_time() {
            let input = "a 2\n0 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res, Err(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_parse_n() {
            let input = "1 a\n0 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res, Err(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_0_seconds() {
            let input = "0 2\n0 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res, Err(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_not_enough_lines() {
            let input = "1 2\n0 1";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res, Err(SolverCreationError::NotEnoughLines))
        }

        #[test]
        fn create_err_too_many_lines() {
            let input = "1 2\n0 1\n1 0\n1 1";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res, Err(SolverCreationError::TooManyLines))
        }

        #[test]
        fn create_err_line_parse() {
            let input = "1 2\na 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res, Err(SolverCreationError::InvalidLine))
        }

        #[test]
        fn create_err_line_too_many() {
            let input = "1 2\n0 1 2\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res, Err(SolverCreationError::InvalidLine))
        }

        #[test]
        fn create_err_line_not_enough() {
            let input = "1 2\n0\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res, Err(SolverCreationError::InvalidLine))
        }
    }
}
