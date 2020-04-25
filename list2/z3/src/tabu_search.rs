use crate::board::Board;
use crate::point::Point;
use crate::tabu_search::direction::{Direction, DIRECTIONS};
use crate::tabu_search::solution::{PartialPath, Solution, SolutionWithCost};
use itertools::Itertools;
use rand::distributions::Standard;
use rand::prelude::*;
use std::collections::BTreeSet;
use std::convert::{identity, TryInto};
use std::time::{Duration, Instant};

pub(crate) mod direction;
mod solution;

fn starting_solution(board: &Board) -> SolutionWithCost {
    let (h, w) = board.fields.dim();
    let mut moves = Vec::with_capacity(h * w);
    let mut current_pos = board.agent_position;

    let dirs = DIRECTIONS;

    for (check_dir, move_dir) in dirs.iter().cycle().tuple_windows() {
        loop {
            let check: Point = check_dir.move_point(current_pos);
            if board.is_exit(check) {
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

struct Path {
    moves: Vec<Direction>,
    ending_point: Point,
}

fn generate_valid_path(
    starting_point: Point,
    board: &Board,
    max_length: usize,
) -> (PartialPath, bool) {
    let moves = thread_rng()
        .sample_iter(Standard)
        .take(max_length)
        .collect();
    PartialPath::verify(starting_point, moves, board)
}

fn generate_path_to_exit(board: &Board) -> Path {
    let current_pos = board.agent_position;
    let (h, w) = board.fields.dim();

    loop {
        let (partial_path, exited) = generate_valid_path(current_pos, board, h + w);
        todo!();
    }
}

pub fn search(board: &Board, time_limit: Duration) -> SolutionWithCost {
    let start_time = Instant::now();

    let (h, w) = board.fields.dim();
    let tabu_size = usize::pow((h + w) as _, 2);
    let mut tabu = BTreeSet::new();

    let mut tmp_vec = Vec::with_capacity(tabu_size);

    let mut current = starting_solution(board);
    eprintln!("initial solution: {:?}", current);
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

    let mut fails = 0;

    for _ in limiter {
        let neighbours = current
            .solution
            .neighbours_swap(best_global.cost)
            .take(tabu_size * tabu_size)
            .filter(|s| !tabu.contains(s))
            .take(tabu_size);

        tmp_vec.clear();
        tmp_vec.extend(
            neighbours
                .map(|s| s.remove_redundancies())
                .map(|s| s.into_solution_with_cost(board)),
        );

        let best = tmp_vec
            .iter()
            .filter_map(|s| s.as_ref().ok())
            .min_by_key(|s| s.cost);

        if let Some(best) = best {
            if best.cost < current.cost || (best.cost == current.cost && thread_rng().gen_bool(0.3))
            {
                current = best.clone();
            }
            if current.cost < best_global.cost {
                eprintln!("{:?} -> {:?}", best_global.cost, current.cost);
                best_global = current.clone();
            } else {
                fails += 1;
            }
            tabu.clear();
        } else {
            fails += 5;
        }

        if fails > std::cmp::min(h, w) {
            return best_global;
        }

        tabu.extend(
            tmp_vec
                .drain(..)
                .map(|s| s.map(|s| s.into_inner()).unwrap_or_else(identity)),
        );
    }

    best_global
}
