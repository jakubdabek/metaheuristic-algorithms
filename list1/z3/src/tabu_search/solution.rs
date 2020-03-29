use super::direction::Direction;
use crate::board::Board;

struct Solution {
    moves: Vec<Direction>,
}

impl Solution {
    fn into_solution_with_cost(self, board: &Board) -> Result<SolutionWithCost, Solution> {
        let mut current_pos = board.agent_position;
        for (count, &dir) in self.moves.iter().enumerate() {
            let new_pos = dir.move_point(current_pos);
            if !board.in_bounds(new_pos) {
                return Err(self);
            }

            if current_pos == board.goal_position {
                let mut moves = self.moves;
                moves.truncate(count + 1);
                let cost = moves.len();

                return Ok(SolutionWithCost { moves, cost });
            }

            current_pos = new_pos;
        }

        let cost = if current_pos == board.goal_position {
            self.moves.len()
        } else {
            std::usize::MAX
        };

        Ok(SolutionWithCost {
            moves: self.moves,
            cost,
        })
    }
}

type Cost = usize;

struct SolutionWithCost {
    moves: Vec<Direction>,
    cost: Cost,
}
