use self::solver::Solver;
use std::error::Error;
use std::io::stdin;

mod solver;

fn main() -> Result<(), Box<dyn Error>> {
    let solver = Solver::try_from_read(stdin().lock())?;

    let solution = solver.search();

    println!("{}", solution.distance);

    for row in solution.matrix.outer_iter() {
        for value in row {
            eprint!("{} ", value);
        }
        eprintln!();
    }

    Ok(())
}
