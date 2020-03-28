use std::io::stdin;
use z3::board::{Board, BoardCreationError};

fn main() -> Result<(), BoardCreationError> {
    let board = Board::try_from_read(stdin().lock())?;

    eprintln!("{:?}", board);

    Ok(())
}
