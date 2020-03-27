use crate::problem::{Domain, Problem};
use std::cmp::Ordering;
use std::time::Instant;

mod common;
pub mod griewank;
pub mod happy_cat;
pub mod problem;

type Value<P> = assoc_fcs!(P: Problem->Domain->Value);
type Argument<P> = assoc_fcs!(P: Problem->Domain->Argument);

fn cmp_partial<T: PartialOrd>(a: &T, b: &T) -> Ordering {
    if let Some(ordering) = a.partial_cmp(b) {
        ordering
    } else {
        Ordering::Equal
    }
}

pub fn search<P: Problem>(time_limit: Instant) -> (Argument<P>, Value<P>) {
    let starting_point = P::Domain::random(1.0);
    let mut best = (starting_point.clone(), P::value(&starting_point));
    let mut current = starting_point;

    fn find_next<P: Problem>(
        current: &Argument<P>,
        best_value: &Value<P>,
    ) -> Option<(Argument<P>, Value<P>)> {
        let neighbours = std::iter::repeat_with(|| {
            let next = P::Domain::random_near(current, 0.5);
            let next_value = P::value(&next);
            (next, next_value)
        });
        let better = neighbours
            .take(5000)
            .filter(|(_, next_value)| next_value < best_value)
            .take(20);

        better.max_by(|(_, a), (_, b)| cmp_partial(a, b))
    }

    while Instant::now() < time_limit {
        if let Some((next, next_value)) = find_next::<P>(&current, &best.1) {
            best = (next.clone(), next_value);
            current = next;
        } else {
            break;
        }
    }

    best
}
