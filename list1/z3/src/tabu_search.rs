use crate::board::Board;
use crate::point::Point;
use crate::tabu_search::direction::{Direction, DIRECTIONS};
use crate::tabu_search::solution::{Solution, SolutionWithCost};
use itertools::Itertools as _;
use rand::distributions::Uniform;
use rand::{random, thread_rng, Rng};
use std::collections::{BTreeSet, VecDeque};
use std::convert::{identity, TryInto};
use std::time::{Duration, Instant};

pub(crate) mod direction;
mod solution;

fn starting_solution(board: &Board) -> SolutionWithCost {
    let mut moves = Vec::with_capacity((board.bounds.x * board.bounds.y).try_into().unwrap());
    let mut current_pos = board.agent_position;

    let dirs = DIRECTIONS;

    for (check_dir, move_dir) in dirs.iter().cycle().tuple_windows() {
        loop {
            let check: Point = check_dir.move_point(current_pos);
            if check == board.goal_position {
                moves.push(*check_dir);
                let cost = moves.len().try_into().unwrap();
                return SolutionWithCost {
                    solution: Solution { moves },
                    cost,
                };
            }

            let move_point = move_dir.move_point(current_pos);
            if !board.in_bounds(move_point) {
                break;
            }

            moves.push(*move_dir);
            current_pos = move_point;
        }
    }

    unreachable!("Can't leave until a solution is found")
}

pub fn search(board: &Board, time_limit: Duration) -> SolutionWithCost {
    let start_time = Instant::now();

    let tabu_size = usize::pow((board.bounds.x + board.bounds.y).try_into().unwrap(), 2);
    let mut tabu = BTreeSet::new();

    let mut tmp_vec = Vec::with_capacity(tabu_size.try_into().unwrap());

    let mut current = starting_solution(board);
    // eprintln!("initial solution: {:?}", current);
    let mut best_global = current.clone();

    let mut iters = 1;
    let limiter = std::iter::from_fn(|| {
        let elapsed = start_time.elapsed();
        eprintln!("{:?}, {:?}", elapsed, elapsed / iters);
        iters += 1;

        if elapsed < time_limit {
            Some(())
        } else {
            None
        }
    });

    for _ in limiter {
        let neighbours = current
            .solution
            .neighbours_swap(best_global.cost)
            .take(tabu_size * tabu_size)
            .filter(|s| !tabu.contains(s))
            .take(tabu_size);

        tmp_vec.clear();
        tmp_vec.extend(neighbours.map(|s| s.into_solution_with_cost(board)));

        let best = tmp_vec
            .iter()
            .filter_map(|s| s.as_ref().ok())
            .min_by_key(|s| s.cost);

        if let Some(best) = best {
            if best.cost < best_global.cost
                || (best.cost == best_global.cost && thread_rng().gen_bool(0.3))
            {
                best_global = best.clone();
            }
            tabu.clear();
        }

        tabu.extend(
            tmp_vec
                .drain(..)
                .map(|s| s.map(|s| s.into_inner()).unwrap_or_else(identity)),
        )
    }

    best_global
}
