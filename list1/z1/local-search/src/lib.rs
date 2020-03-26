use crate::problem::{Domain, Problem};

mod common;
mod griewank;
mod happy_cat;
mod problem;

fn search<P: Problem>() {
    let starting_point = P::Domain::random(1.0);
    let mut best = P::value(starting_point);
}
