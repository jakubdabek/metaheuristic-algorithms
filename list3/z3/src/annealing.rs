use crate::board::Board;
use crate::direction::Direction;
use crate::path::Path;
use crate::utils;
use rand::prelude::*;
use std::time::{Duration, Instant};

fn randomly_better(current: u64, next: u64, temperature: f64, rng: &mut impl Rng) -> bool {
    f64::exp((next as f64 - current as f64) / temperature) < rng.gen()
}

pub fn search(
    board: &Board,
    _: Option<(u64, Vec<Vec<Direction>>)>,
    time_limit: Duration,
) -> Vec<Direction> {
    let start_time = Instant::now();

    let (h, w) = board.fields.dim();

    let mut current = Path::new_to_exit(board);
    eprintln!("initial solution: {:?}", current.moves.len());
    current.remove_redundancies();
    eprintln!("initial solution: {:?}", current.moves.len());

    let mut best_global = current.clone();

    let limiter = utils::make_limiter_debug(start_time, time_limit, 10);

    let rng = &mut thread_rng();
    let mut fails = 0;
    let mut temperature = 273.15;

    for () in limiter {
        let mut next = current.neighbour_by_swap_extend(best_global.get_cost(), board);
        next.remove_redundancies();
        let next_cost = next.get_cost();

        if next_cost < current.get_cost()
            || randomly_better(current.get_cost(), next_cost, temperature, rng)
        {
            eprintln!("{:?} -> {:?}", current.get_cost(), next.get_cost());
            current = next;

            fails /= 2;

            if current.get_cost() < best_global.get_cost() {
                eprintln!("{:?} => {:?}", best_global.get_cost(), current.get_cost());
                best_global = current.clone();
                fails /= 4;
            }
        } else {
            fails += 1;
        }

        if fails as f32 > f32::powf((h + w) as _, 1.6) {
            break;
        }

        temperature *= 0.98;
    }

    best_global.moves
}
