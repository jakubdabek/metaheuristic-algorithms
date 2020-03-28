use ::z2::tsp_tabu_search::Solver;
use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let solver = Solver::try_from_read(stdin().lock())?;

    let solution = solver.search();
    println!("{}", solution.cost);
    eprintln!("{:?}", solution.path);

    Ok(())
}
