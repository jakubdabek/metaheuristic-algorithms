use crate::population::specimen::{Specimen, GENOME_LENGTH, MAX_GENOME_VALUE};
use rand::distributions::{Bernoulli, Uniform};
use rand::prelude::*;
use std::cmp::Ordering;
use std::time::Instant;

pub mod specimen;

#[derive(Debug, Clone, PartialEq)]
pub struct Solution {
    pub specimen: Specimen,
    pub argument: Argument,
    pub value: Value,
}

impl Solution {
    fn new(specimen: &Specimen, argument: Argument, value: Value) -> Self {
        Self { specimen: specimen.clone(), argument, value }
    }
}

pub type Scalar = f64;

pub const SIZE: usize = 5;

pub type Argument = [Scalar; SIZE];
pub type Value = f64;

pub struct XsYang {
    parameters: Argument,
}

impl XsYang {
    pub fn new(parameters: Argument) -> Self {
        assert!(parameters.iter().all(|&x| x >= 0.0 && x <= 1.0));

        Self { parameters }
    }

    pub fn value(&self, argument: &Argument) -> Value {
        argument
            .iter()
            .zip(self.parameters.iter())
            .zip(1..)
            .map(|((arg, param), i)| param * arg.abs().powi(i))
            .sum()
    }
}

struct SpecimenWithValue {
    specimen: Specimen,
    value: Value,
}

impl SpecimenWithValue {
    fn new(specimen: Specimen, fitness: &XsYang) -> Self {
        let value = fitness.value(&specimen.to_argument());

        Self { specimen, value }
    }

    fn from_argument(argument: &Argument, fitness: &XsYang) -> Self {
        let value = fitness.value(argument);

        Self {
            specimen: Specimen::from_argument(argument),
            value,
        }
    }
}

const GENERATION_SIZE: usize = 5000;

#[derive(Debug, PartialOrd, PartialEq)]
struct AssertOrd<T> {
    value: T,
}

impl<T: PartialEq> Eq for AssertOrd<T> {}

impl<T: PartialOrd> Ord for AssertOrd<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("tried to compare non-comparable values")
    }
}

fn assert_ord<T: PartialOrd>(value: T) -> AssertOrd<T> {
    AssertOrd { value }
}

pub fn search(starting_point: Argument, fitness: XsYang, time_limit: Instant) -> Solution {
    let mut population = Vec::with_capacity(GENERATION_SIZE * 2);
    population.push(SpecimenWithValue::from_argument(&starting_point, &fitness));

    let rng = &mut thread_rng();
    let genome_value_dist = Uniform::new_inclusive(0, MAX_GENOME_VALUE);
    population.extend(
        std::iter::repeat_with(|| {
            let mut values = [0; SIZE];
            for v in &mut values {
                *v = rng.sample(genome_value_dist);
            }
            SpecimenWithValue::new(Specimen::new(values), &fitness)
        })
        .take(GENERATION_SIZE - 1),
    );

    let mut probabilities = Vec::with_capacity(population.capacity());
    let mut chosen_pairs = Vec::with_capacity(population.capacity());
    let mut tmp = Vec::with_capacity(population.capacity());
    let mut mutations = Vec::with_capacity(GENOME_LENGTH as _);

    let genome_length_dist = Uniform::new(0, GENOME_LENGTH);
    let recombination_choice_dist = Bernoulli::from_ratio(1, 3).unwrap();
    let genome_size_dist = Uniform::new(0, SIZE);
    let mutation_choice_dist = Bernoulli::from_ratio(1, 10).unwrap();
    let genome_mutation_probability_dist = Bernoulli::from_ratio(1, 1000).unwrap();

    let mut iters = 0;
    while Instant::now() < time_limit {
        population.sort_unstable_by_key(|s| assert_ord(s.value));
        population.truncate(GENERATION_SIZE);

        probabilities.clear();
        probabilities.extend(population.iter().scan(0.0, |acc, s| {
            *acc += s.value;
            Some(*acc)
        }));

        let sum = probabilities.last().unwrap();
        let dist = Uniform::new_inclusive(0.0, sum);
        chosen_pairs.clear();
        for _ in 0..GENERATION_SIZE / 2 {
            let s1 = probabilities
                .binary_search_by_key(&assert_ord(&rng.sample(dist)), assert_ord)
                .unwrap_or_else(|x| x);
            let s2 = probabilities
                .binary_search_by_key(&assert_ord(&rng.sample(dist)), assert_ord)
                .unwrap_or_else(|x| x);
            chosen_pairs.push((s1, s2));
        }

        for &(s1, s2) in &chosen_pairs {
            let s1 = &population[s1].specimen;
            let s2 = &population[s2].specimen;

            let specimen = if rng.sample(recombination_choice_dist) {
                let i = rng.sample(genome_size_dist);
                Specimen::recombine_outer(s1, s2, i)
            } else {
                let i = rng.sample(genome_length_dist);
                let j = rng.gen_range(i, GENOME_LENGTH);

                Specimen::recombine_inner(s1, s2, i, j)
            };
            population.push(SpecimenWithValue::new(specimen, &fitness))

        }

        tmp.clear();
        tmp.extend(population.iter().filter_map(|s| {
            if rng.sample(mutation_choice_dist) {
                if rng.sample(genome_mutation_probability_dist) {
                    Some(SpecimenWithValue::new(
                        Specimen::mutate_big(&s.specimen, rng.sample(genome_size_dist)),
                        &fitness,
                    ))
                } else {
                    None
                }
            } else {
                mutations.clear();
                for i in 0..GENOME_LENGTH {
                    if rng.sample(genome_mutation_probability_dist) {
                        mutations.push(i);
                    }
                }
                if !mutations.is_empty() {
                    Some(SpecimenWithValue::new(
                        Specimen::mutate_small(&s.specimen, &mutations),
                        &fitness,
                    ))
                } else {
                    None
                }
            }
        }));

        population.extend(tmp.drain(..));

        iters += 1;
    }

    println!("{}", iters);
    let best = population
        .iter()
        .min_by_key(|&s| assert_ord(&s.value))
        .unwrap();
    Solution::new(&best.specimen, best.specimen.to_argument(), best.value)
}
