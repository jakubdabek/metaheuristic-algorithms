use crate::board::Board;
use crate::tabu_search::direction::Direction;
use crate::tabu_search::solution::Path;
use rand::prelude::*;
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

pub(crate) mod direction;
mod solution;

pub fn search(board: &Board, time_limit: Duration) -> Vec<Direction> {
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
        let neighbours =
            std::iter::repeat_with(|| current.neighbour_by_swap_extend(best_global.cost(), board));
        let neighbours = neighbours
            .take(tabu_size * tabu_size)
            .filter(|s| !tabu.contains(s))
            .take(tabu_size);

        tmp_vec.clear();
        tmp_vec.extend(neighbours.map(|mut s| {
            s.remove_redundancies();
            s
        }));

        let best = tmp_vec.iter().min_by_key(|s| s.cost());

        if let Some(best) = best {
            if best.cost() < current.cost()
                || (best.cost() == current.cost() && thread_rng().gen_bool(0.3))
            {
                current = best.clone();
            }
            if current.cost() < best_global.cost() {
                eprintln!("{:?} -> {:?}", best_global.cost(), current.cost());
                fails = fails / 2;
                best_global = current.clone();
            } else {
                fails += 1;
            }
            tabu.clear();
        } else {
            fails += 5;
        }

        if fails > std::cmp::min(h, w) {
            return best_global.moves;
        }

        tabu.extend(tmp_vec.drain(..));
    }

    best_global.moves
}
