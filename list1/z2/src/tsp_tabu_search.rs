use itertools::Itertools;
use ndarray::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::{BTreeSet, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::io::BufRead;
use std::time::{Duration, Instant};

mod path;

type Cost = usize;
type NodeIndex = usize;
type CostMatrix = Array2<Cost>;
type TabuCollection = BTreeSet<path::Path>;

#[derive(Debug, Clone)]
pub struct Solver {
    distances: CostMatrix,
    time_limit: Duration,
}

pub struct Solution {
    pub path: Vec<NodeIndex>,
    pub cost: usize,
}

impl Solver {
    pub fn new(distances: CostMatrix, time_limit: Duration) -> Self {
        Self {
            distances,
            time_limit,
        }
    }

    pub fn search(&self) -> Solution {
        use path::*;
        let start_time = Instant::now();

        let costs = &self.distances;
        let n = costs.ncols();
        // let mut tabu = TabuCollection::with_capacity(100 * n);
        let mut tabu = TabuCollection::new();
        let mut outer_tabu = TabuCollection::new();

        let mut best = {
            let first = Path::new_random(n);
            PathWithCost::from_path(first, costs)
        };

        let mut iters = 1;

        let local_starts = std::iter::once(best.clone()).chain(std::iter::from_fn(|| {
            let elapsed = start_time.elapsed();
            eprintln!("{:?}, {:?}", elapsed, elapsed / iters);
            iters += 1;

            if elapsed < self.time_limit {
                let new = Path::new_random(n);
                Some(PathWithCost::from_path(new, costs))
            } else {
                None
            }
        }));

        let mut tmp_vec = Vec::<PathWithCost>::with_capacity(n * (n - 1) / 2);

        for local_start in local_starts {
            eprintln!("{}", tabu.len());
            tabu.clear();
            tabu.insert(local_start.inner().clone());
            let mut current = local_start;
            let mut current_best = current.clone();
            for _ in 0..2000 {
                let neighbourhood = (1..n)
                    .tuple_combinations()
                    .map(|(i, j)| current.neighbour_swap_nodes(i, j))
                    .filter(|p| !outer_tabu.contains(p.as_path()))
                    .filter(|p| !tabu.contains(p.as_path()));

                tmp_vec.clear();
                tmp_vec.extend(neighbourhood.map(|nsn| nsn.into_path_with_cost(costs)));

                let local_opt = tmp_vec.iter().min_by_key(|p| p.cost()).cloned();

                let randoms = thread_rng().sample_iter(Uniform::new_inclusive(0.0, 1.0));
                let tba = tmp_vec
                    .drain(..)
                    .zip(randoms)
                    .filter(|(_, rand_val)| rand_val < &0.3)
                    .map(|(p, _)| p.into_inner());
                tabu.extend(tba);

                if let Some(local_opt) = local_opt {
                    if local_opt.cost() < current_best.cost() {
                        current_best = local_opt.clone();
                    }
                    current = local_opt.clone();
                    tabu.insert(local_opt.into_inner());
                } else {
                    break;
                }
            }

            if current.cost() < best.cost() {
                best = current;
                outer_tabu.insert(best.inner().clone());
            }
        }

        best.into_solution()
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

        let (time, n) = match *header.as_slice() {
            [time, n] if time > 0 => (time, n),
            _ => return Err(InvalidHeader),
        };

        let mut matrix = CostMatrix::zeros((n, n));

        for zipped in matrix.outer_iter_mut().zip_longest(lines) {
            use itertools::EitherOrBoth::*;
            let (mut row, line) = match zipped {
                Both(row, line) => (row, line),
                Left(_) => return Err(NotEnoughLines),
                Right(_) => return Err(TooManyLines),
            };

            let line = line?;
            let values = line.split_ascii_whitespace().map(str::parse::<Cost>);

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
            assert_eq!(res.err(), Some(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_parse_n() {
            let input = "1 a\n0 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_0_seconds() {
            let input = "0 2\n0 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidHeader))
        }

        #[test]
        fn create_err_not_enough_lines() {
            let input = "1 2\n0 1";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::NotEnoughLines))
        }

        #[test]
        fn create_err_too_many_lines() {
            let input = "1 2\n0 1\n1 0\n1 1";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::TooManyLines))
        }

        #[test]
        fn create_err_line_parse() {
            let input = "1 2\na 1\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidLine))
        }

        #[test]
        fn create_err_line_too_many() {
            let input = "1 2\n0 1 2\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidLine))
        }

        #[test]
        fn create_err_line_not_enough() {
            let input = "1 2\n0\n1 0";
            let res = Solver::try_from_read(Cursor::new(input));
            assert_eq!(res.err(), Some(SolverCreationError::InvalidLine))
        }
    }
}
