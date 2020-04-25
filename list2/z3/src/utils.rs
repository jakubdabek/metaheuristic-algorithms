#![allow(dead_code)]
use std::time::{Duration, Instant};

pub(crate) fn make_limiter_debug(
    start_time: Instant,
    time_limit: Duration,
    cycle_num: u32,
) -> impl Iterator<Item = ()> {
    make_limiter_impl(start_time, time_limit, cycle_num, true)
}

pub(crate) fn make_limiter(
    start_time: Instant,
    time_limit: Duration,
    cycle_num: u32,
) -> impl Iterator<Item = ()> {
    make_limiter_impl(start_time, time_limit, cycle_num, false)
}

fn make_limiter_impl(
    start_time: Instant,
    time_limit: Duration,
    cycle_num: u32,
    debug: bool,
) -> impl Iterator<Item = ()> {
    std::iter::from_fn({
        let mut iters = 1;
        move || {
            let elapsed = start_time.elapsed();
            if debug && iters % cycle_num == 0 {
                eprintln!(
                    "{:12} iters in {:.6?}, avg {:6.3?}",
                    iters,
                    elapsed,
                    elapsed / iters
                );
            }
            iters += 1;
            if elapsed < time_limit {
                Some(())
            } else {
                None
            }
        }
    })
}
