use crate::board::Board;
use crate::direction::Direction;
use std::time::Duration;

pub fn search(
    board: &Board,
    initial_solutions: Option<(u64, Vec<Vec<Direction>>)>,
    time_limit: Duration,
) -> Vec<Direction> {
    todo!(
        "{:?} {:?} {:?}",
        board.fields.dim(),
        initial_solutions,
        time_limit
    )
}
