use crate::board::Board;
use crate::direction::Direction;
use crate::path::Path;
use crate::utils;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::time::{Duration, Instant};

const GENERATION_SIZE: usize = 20;

pub fn search(
    board: &Board,
    initial_solutions: Option<(u64, Vec<Vec<Direction>>)>,
    time_limit: Duration,
) -> Vec<Direction> {
    let start_time = Instant::now();

    // let (h, w) = board.fields.dim();

    let (generation_size, mut population) = if let Some((p, initial)) = initial_solutions {
        let size = usize::min(GENERATION_SIZE, p as _);

        let mut population = Vec::with_capacity(size * 3);
        population.extend(
            initial
                .into_iter()
                .map(|moves| Path::new(board.agent_position, moves, board)),
        );

        (size, population)
    } else {
        let mut initial = Path::new_to_exit(board);
        initial.remove_redundancies();

        (GENERATION_SIZE, vec![initial])
    };

    let limiter = utils::make_limiter_debug(start_time, time_limit, 10);

    let rng = &mut thread_rng();

    let mut chosen_pairs = Vec::with_capacity(generation_size);

    for () in limiter {
        chosen_pairs.clear();
        let population_dist = Uniform::new(0, population.len());
        for _ in 0..generation_size {
            chosen_pairs.push((rng.sample(population_dist), rng.sample(population_dist)));
        }

        for &(p1, p2) in &chosen_pairs {
            let p1: &Path = &population[p1];
            let p2 = &population[p2];

            let start1 = rng.gen_range(0, p1.moves.len());
            let start2 = rng.gen_range(0, p2.moves.len() - 1);
            let end2 = rng.gen_range(start2 + 1, p2.moves.len());
            let combined = Path::recombine_splice(p1, p2, start1, start2, end2, board);
            population.push(combined);
        }

        todo!("mutations")
    }

    todo!("return result")
}
