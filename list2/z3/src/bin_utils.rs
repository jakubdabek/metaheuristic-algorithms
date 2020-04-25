use std::io::stdin;
use crate::board::{BoardCreationError, Board};
use std::time::Duration;
use crate::direction::Direction;

pub type Result = std::result::Result<(), BoardCreationError>;

pub fn main(f: impl FnOnce(&Board, Duration) -> Vec<Direction>) -> Result {
    let (board, time_limit) = Board::try_from_read(stdin().lock())?;

    eprintln!("{:?}", board);

    let solution = f(&board, time_limit);

    println!("{}", solution.len());
    eprintln!("{:?}", solution);

    Ok(())
}
