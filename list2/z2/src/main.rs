#![allow(clippy::unit_arg)]

use self::solver::Solver;
use std::error::Error;
use std::io::stdin;
use ndarray::ArrayView2;
use crate::solver::Value;

mod solver;

fn print_mat(arr: ArrayView2<Value>) {
    for row in arr.outer_iter() {
        for value in row {
            eprint!("{:3} ", value);
        }
        eprintln!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let solver = Solver::try_from_read(stdin().lock())?;

    let (blocks, full_solution) = solver.search();

    println!("{}", full_solution.distance);

    print_mat(blocks.values.view());
    // print_mat(full_solution.matrix.view());

    Ok(())
}
