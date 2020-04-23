use crate::solver::block_matrix::BlockMatrix;
use itertools::Itertools;
use ndarray::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::convert::TryInto;
use std::fmt;
use std::io::BufRead;
use std::time::{Duration, Instant};

type Value = u8;
type ValueMatrix = Array2<Value>;
type Distance = f64;

mod block_matrix;

#[derive(Debug, Clone)]
pub struct Solver {
    values: ValueMatrix,
    minimal_block_size: usize,
    time_limit: Duration,
}

#[derive(Debug, Clone)]
struct Solution {
    matrix: BlockMatrix,
    distance: Distance,
}

#[derive(Debug, Clone)]
pub struct FullSizeSolution {
    pub matrix: ValueMatrix,
    pub distance: Distance,
}

impl Solution {
    pub fn new(matrix: BlockMatrix, distance: Distance) -> Self {
        Self { matrix, distance }
    }

    fn to_full_size(&self, dim: Ix2) -> FullSizeSolution {
        FullSizeSolution {
            matrix: self.matrix.to_full_size(dim),
            distance: self.distance,
        }
    }
}

impl Solver {
    pub fn new(values: ValueMatrix, block_size: usize, time_limit: Duration) -> Self {
        Self {
            values,
            minimal_block_size: block_size,
            time_limit,
        }
    }

    fn randomly_better(current: Distance, next: Distance, temperature: f64, rng: &mut impl Rng) -> bool {
        f64::exp((next - current) / temperature) < rng.gen()
    }

    pub fn search(&self) -> FullSizeSolution {
        let start_time = Instant::now();

        let (h, w) = self.values.dim();
        let values = &self.values;

        let initial = BlockMatrix::zeros(self.minimal_block_size, self.minimal_block_size, h, w);
        let initial_distance = initial.distance_from(values);
        let mut best = Solution::new(
            initial,
            initial_distance,
        );

        let mut current = best.clone();

        let mut rng = thread_rng();
        let block_height_dist = Uniform::new(self.minimal_block_size, h);
        let block_width_dist = Uniform::new(self.minimal_block_size, w);

        let mut temperature = 273.15;

        let iter = std::iter::from_fn({
            let time_limit = self.time_limit;
            let mut iters = 1;
            move || {
                let elapsed = start_time.elapsed();
                if iters % 10000 == 0 {
                    eprintln!("{:12} iters in {:.6?}, avg {:6.3?}", iters, elapsed, elapsed / iters);
                }
                iters += 1;
                if elapsed < time_limit {
                    Some(())
                } else {
                    None
                }
            }
        });

        for () in iter {
            let next = if rng.gen_bool(0.01) {
                current.matrix.with_block_size(
                    rng.sample(block_height_dist),
                    rng.sample(block_width_dist),
                    h,
                    w,
                )
            } else {
                current.matrix.clone().perturb_values()
            };

            let next_distance = next.distance_from(values);
            if next_distance < current.distance || Self::randomly_better(current.distance, next_distance, temperature, &mut rng) {
                current = Solution::new(next, next_distance);
                if current.distance < best.distance {
                    best = current.clone();
                }
            }

            temperature *= 0.9;
        }

        best.to_full_size(values.raw_dim())
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
