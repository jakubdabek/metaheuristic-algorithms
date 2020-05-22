use crate::board::Board;
use crate::direction::Direction;
use crate::path::Path;
use crate::utils;
use rand::prelude::*;
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

pub fn search(
    board: &Board,
    _: Option<(u64, Vec<Vec<Direction>>)>,
    time_limit: Duration,
) -> Vec<Direction> {
    let start_time = Instant::now();

    let (h, w) = board.fields.dim();
    let tabu_size = usize::pow(std::cmp::min(h, w) as _, 1);
    let mut tabu = BTreeSet::new();

    let mut tmp_vec = Vec::with_capacity(tabu_size);

    let mut current = Path::new_to_exit(board);
    eprintln!("initial solution: {:?}", current.moves.len());
    current.remove_redundancies();
    eprintln!("initial solution: {:?}", current.moves.len());
    let mut best_global = current.clone();

    let limiter = utils::make_limiter_debug(start_time, time_limit, 2);

    let mut fails = 0;

    for () in limiter {
        let tabu_size = f64::max(
            1.0,
            tabu_size as f64 * tabu_size as f64 / best_global.get_cost() as f64,
        );
        let tabu_size = tabu_size as _;
        let neighbours =
            std::iter::repeat_with(|| current.neighbour_by_swap_extend(tabu_size as _, board));
        let neighbours = neighbours
            .take(tabu_size * tabu_size)
            .filter(|s| !tabu.contains(s))
            .take(tabu_size);

        tmp_vec.clear();
        tmp_vec.extend(neighbours.map(|mut s| {
            s.remove_redundancies();
            s
        }));

        let best = tmp_vec.iter().min_by_key(|s| s.get_cost());

        if let Some(best) = best {
            if best.get_cost() < current.get_cost()
                || (best.get_cost() == current.get_cost() && thread_rng().gen_bool(0.3))
            {
                current = best.clone();
            }
            if current.get_cost() < best_global.get_cost() {
                eprintln!("{:?} -> {:?}", best_global.get_cost(), current.get_cost());
                fails /= 2;
                best_global = current.clone();
            } else {
                fails += 1;
            }
            tabu.clear();
        } else {
            fails += 5;
        }

        if fails > h + w {
            return best_global.moves;
        }

        tabu.extend(tmp_vec.drain(..));
    }

    best_global.moves
}
