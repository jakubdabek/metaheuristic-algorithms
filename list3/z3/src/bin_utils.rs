use crate::board::{Board, BoardCreationError};
use crate::direction::Direction;
use std::io::stdin;
use std::time::Duration;

pub type Result = std::result::Result<(), BoardCreationError>;

pub fn main(
    f: impl FnOnce(&Board, Option<(u64, Vec<Vec<Direction>>)>, Duration) -> Vec<Direction>,
) -> Result {
    let (board, initial_solutions, time_limit) = Board::try_from_read(stdin().lock())?;

    // eprintln!("{:?}", board);

    let solution = f(&board, initial_solutions, time_limit);

    println!("{}", solution.len());
    eprintln!("{:?}", solution);

    Ok(())
}
