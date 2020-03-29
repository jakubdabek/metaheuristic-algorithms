use std::io::stdin;
use z3::board::{Board, BoardCreationError};
use z3::tabu_search;

fn main() -> Result<(), BoardCreationError> {
    let (board, time_limit) = Board::try_from_read(stdin().lock())?;

    eprintln!("{:?}", board);

    let solution = tabu_search::search(&board, time_limit);

    println!("{}", solution.cost);
    eprintln!("{:?}", solution.solution.moves);

    Ok(())
}
