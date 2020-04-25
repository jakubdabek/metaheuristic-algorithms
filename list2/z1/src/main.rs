use crate::annealing::salomon::{Argument, Scalar, Value};
use crate::annealing::Solution;
use std::io::Read;
use std::ops::Add;
use std::time::{Duration, Instant};

mod annealing;

fn do_search(initial: Argument, duration_limit: Duration) -> (Argument, Value, Duration) {
    let start = Instant::now();
    let Solution {
        argument: arg,
        value: val,
    } = annealing::search(initial, Instant::now().add(duration_limit));

    let elapsed = Instant::now().duration_since(start);

    (arg, val, elapsed)
}

fn main_interactive() -> Result<(), String> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Reading stdin failed");

    let parsed = input
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()
        .map_err(|e| e.to_string())?;

    let (time, initial) = match *parsed.as_slice() {
        [time, x1, x2, x3, x4] if time > 0 => (
            time,
            Argument::new(x1 as Scalar, x2 as Scalar, x3 as Scalar, x4 as Scalar),
        ),
        _ => return Err(String::from("Incorrect arguments")),
    };

    let (arg, val, _elapsed) = do_search(initial, Duration::from_secs(time as u64));

    for x in arg.iter() {
        print!("{} ", x);
    }

    print!("{}", val);

    Ok(())
}

fn main() -> Result<(), String> {
    main_interactive()
}
