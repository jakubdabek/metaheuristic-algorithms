use self::salomon::{Argument, Value};
use rand::prelude::*;
use std::time::Instant;

pub mod salomon;

#[derive(Debug, Clone, PartialEq)]
pub struct Solution {
    pub argument: Argument,
    pub value: Value,
}

impl Solution {
    fn new(argument: Argument, value: Value) -> Self {
        Self { argument, value }
    }
}

fn randomly_better(current: Value, next: Value, temperature: f64, rng: &mut impl Rng) -> bool {
    f64::exp((next - current) / temperature) < rng.gen()
}

pub fn search(starting_point: Argument, time_limit: Instant) -> Solution {
    let mut best = Solution::new(starting_point, salomon::value(&starting_point));
    let mut current = best.clone();

    let mut temperature = 273.15;

    let rng: &mut ThreadRng = &mut thread_rng();

    while Instant::now() < time_limit {
        let next = salomon::random_near(&current.argument, 0.005 * f64::max(temperature, 1.0));
        let next_value = salomon::value(&next);

        if next_value < current.value
            || randomly_better(current.value, next_value, temperature, rng)
        {
            current = Solution::new(next, next_value);

            if current.value < best.value {
                best = current.clone();
            }
        }

        temperature -= 1.0;
    }

    best
}
