use local_search::griewank::Griewank;
use local_search::happy_cat::HappyCat;
use local_search::problem::{Problem, ProblemArgument, ProblemValue};
use std::ops::Add;
use std::time::{Duration, Instant};

fn do_search<P: Problem>() -> (ProblemArgument<P>, ProblemValue<P>, Duration) {
    let start = Instant::now();
    let (arg, val) = local_search::search::<P>(Instant::now().add(Duration::from_secs(5)));

    let elapsed = Instant::now().duration_since(start);

    (arg, val, elapsed)
}

fn main() {
    let (arg, val, elapsed) = do_search::<HappyCat>();
    println!("elapsed: {:?}", elapsed);
    println!("f({:.4?}) = {}", arg, val);

    let (arg, val, elapsed) = do_search::<HappyCat>();
    println!("elapsed: {:?}", elapsed);
    println!("f({:.4?}) = {}", arg, val);

    let (arg, val, elapsed) = do_search::<Griewank>();
    println!("elapsed: {:?}", elapsed);
    println!("f({:.4?}) = {}", arg, val);

    let (arg, val, elapsed) = do_search::<Griewank>();
    println!("elapsed: {:?}", elapsed);
    println!("f({:.4?}) = {}", arg, val);
}
