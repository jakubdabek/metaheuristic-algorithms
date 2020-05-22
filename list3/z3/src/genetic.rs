use crate::board::Board;
use crate::direction::Direction;
use crate::path::Path;
use crate::utils;
use rand::distributions::Uniform;
use rand::prelude::*;
use rand_distr::{Bernoulli, Normal};
use std::time::{Duration, Instant};

const GENERATION_SIZE: usize = 100;

pub fn search(
    board: &Board,
    initial_solutions: Option<(u64, Vec<Vec<Direction>>)>,
    time_limit: Duration,
) -> Vec<Direction> {
    let start_time = Instant::now();

    let (h, w) = board.fields.dim();

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
        (GENERATION_SIZE, vec![])
    };

    if population.is_empty() {
        let mut initial = Path::new_to_exit(board);
        initial.remove_redundancies();

        population.push(initial);
    }

    let period = 10u32.pow(3 - f32::min(f32::log10((h * w) as _) / 2.0, 3.0) as u32);
    let limiter = utils::make_limiter_debug(start_time, time_limit, period);

    let mut chosen_pairs = Vec::with_capacity(generation_size * 2);
    let mut tmp = Vec::with_capacity(generation_size * 2);

    let rng = &mut thread_rng();
    let mutation_probability_dist = Bernoulli::from_ratio(1, 20).unwrap();

    let mut best_cost = population.first().map(|p| p.get_cost()).unwrap();

    for () in limiter {
        population.sort_by_key(|p| p.get_cost());
        let current_best = population.first().map(|p| p.get_cost()).unwrap();
        if current_best < best_cost {
            eprintln!("{:?} => {:?}", best_cost, current_best);
            best_cost = current_best;
        }
        population.truncate(generation_size);

        chosen_pairs.clear();
        let population_dist = Uniform::new(0, population.len());
        for _ in 0..generation_size * 2 {
            chosen_pairs.push((rng.sample(population_dist), rng.sample(population_dist)));
        }

        for &(p1, p2) in &chosen_pairs {
            let p1: &Path = &population[p1];
            let p2 = &population[p2];

            if p1.moves.is_empty() || p2.moves.len() < 2 {
                continue;
            }

            let start1 = rng.gen_range(0, p1.moves.len());
            let start2 = rng.gen_range(0, p2.moves.len() - 1);
            let end2 = rng.gen_range(start2 + 1, p2.moves.len());
            let combined = Path::recombine_splice(p1, p2, start1, start2, end2, board);
            population.push(combined);
        }

        for specimen in &population {
            if !specimen.moves.is_empty() && rng.sample(mutation_probability_dist) {
                let count_dist = Normal::new(specimen.moves.len() as f64 / 2.0, 3.0).unwrap();
                let count = rng
                    .sample_iter(count_dist)
                    .find(|&x| x >= 1.0)
                    .unwrap()
                    .ceil() as _;
                let position_dist = Uniform::new(0, specimen.moves.len());
                tmp.push(Path::mutate_swap(
                    specimen,
                    board,
                    std::iter::repeat_with(|| {
                        (rng.sample(position_dist), rng.sample(position_dist))
                    })
                    .take(count),
                ));
            }
        }
    }

    let best = population.iter().min_by_key(|p| p.get_cost()).unwrap();

    best.moves.clone()
}
