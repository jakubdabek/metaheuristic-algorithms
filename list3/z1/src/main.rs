use crate::population::Solution;
use crate::population::{Argument, Scalar, Value, XsYang};
use crate::population::specimen::Specimen;
use std::array::TryFromSliceError;
use std::convert::TryInto;
use std::io::Read;
use std::ops::Add;
use std::time::{Duration, Instant};

mod population;

fn do_search(
    initial: Argument,
    fitness: XsYang,
    duration_limit: Duration,
) -> (Specimen, Argument, Value, Duration) {
    let start = Instant::now();
    let Solution {
        specimen,
        argument: arg,
        value: val,
    } = population::search(initial, fitness, Instant::now().add(duration_limit));

    let elapsed = Instant::now().duration_since(start);

    (specimen, arg, val, elapsed)
}

fn main_interactive() -> Result<(), String> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Reading stdin failed");

    let mut input = input.split_ascii_whitespace();
    let parsed_ints = input
        .by_ref()
        .take(6)
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()
        .map_err(|e| e.to_string())?;

    let parsed_floats = input
        .map(str::parse)
        .collect::<Result<Vec<f64>, _>>()
        .map_err(|e| e.to_string())?;

    let (time, initial) = match *parsed_ints.as_slice() {
        [time, x1, x2, x3, x4, x5] if time > 0 => (
            time,
            [
                x1 as Scalar,
                x2 as Scalar,
                x3 as Scalar,
                x4 as Scalar,
                x5 as Scalar,
            ],
        ),
        _ => return Err(String::from("Incorrect arguments")),
    };

    let params = parsed_floats
        .as_slice()
        .try_into()
        .map_err(|e: TryFromSliceError| e.to_string())?;

    let (specimen, arg, val, _elapsed) = do_search(
        initial,
        XsYang::new(params),
        Duration::from_secs(time as u64),
    );

    for x in specimen.values.iter() {
        print!("{:b} ", x);
    }

    println!();

    for x in arg.iter() {
        print!("{} ", x);
    }

    print!("{}", val);

    Ok(())
}

fn main() -> Result<(), String> {
    main_interactive()
}
