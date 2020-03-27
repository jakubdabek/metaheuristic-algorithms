use local_search::griewank::Griewank;
use local_search::happy_cat::HappyCat;
use local_search::problem::{Problem, ProblemArgument, ProblemValue};
use std::io::Read;
use std::ops::Add;
use std::time::{Duration, Instant};

#[allow(type_alias_bounds)]
type SearchOk<P: Problem> = (ProblemArgument<P>, ProblemValue<P>, Duration);

fn do_search<P: Problem>(duration_limit: Duration) -> SearchOk<P> {
    let start = Instant::now();
    let (arg, val) = local_search::search::<P>(Instant::now().add(duration_limit));

    let elapsed = Instant::now().duration_since(start);

    (arg, val, elapsed)
}

fn main_interactive() -> Result<(), String> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Reading stdin failed");

    // eprintln!("{:?}", input);

    let parsed = input
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|e| e.to_string())?;

    let (time, choice) = match *parsed.as_slice() {
        [time, choice] if time > 0 => (time, choice),
        _ => return Err(String::from("Incorrect arguments")),
    };

    let (arg, val, _elapsed) = match choice {
        0 => do_search::<HappyCat>(Duration::from_secs(time)),
        1 => do_search::<Griewank>(Duration::from_secs(time)),
        _ => return Err(String::from("Incorrect choice (accepted 0 or 1)")),
    };

    for x in arg.iter() {
        print!("{} ", x);
    }

    print!("{}", val);

    Ok(())
}

fn main() -> Result<(), String> {
    main_interactive()
}
