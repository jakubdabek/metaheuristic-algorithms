use super::direction::Direction;
use crate::board::Board;
use itertools::Itertools as _;
use rand::distributions::Uniform;
use rand::prelude::*;
use rand_distr::Float;
use std::convert::TryInto;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Solution {
    pub moves: Vec<Direction>,
}

impl Solution {
    pub fn neighbours_swap(&self, num: u64) -> impl Iterator<Item = Solution> + '_ {
        let mut rng = thread_rng();
        let dist = Uniform::new(0, self.moves.len());
        let num_dist = rand_distr::Normal::new(num as f64, 4.0).unwrap();
        std::iter::repeat_with(move || {
            let mut new_path = self.moves.clone();
            let num = rng
                .sample_iter(num_dist)
                .filter(|&x| x > 1.0)
                .next()
                .unwrap() as usize;
            for (a, b) in (&mut rng).sample_iter(dist).tuples().take(num) {
                new_path.swap(a, b);
            }

            Solution { moves: new_path }
        })
    }

    pub fn into_solution_with_cost(mut self, board: &Board) -> Result<SolutionWithCost, Solution> {
        let mut current_pos = board.agent_position;
        for (count, &dir) in self.moves.iter().enumerate() {
            if let Some((goal_dir, new_pos)) = board
                .adjacent(current_pos)
                .find(|&(_, p)| p == board.goal_position)
            {
                self.moves.truncate(count);
                self.moves.push(goal_dir);
                let cost = self.moves.len().try_into().unwrap();

                return Ok(SolutionWithCost {
                    solution: self,
                    cost,
                });
            }

            let new_pos = dir.move_point(current_pos);
            if !board.in_bounds(new_pos) {
                return Err(self);
            }

            current_pos = new_pos;
        }

        let cost = if current_pos == board.goal_position {
            self.moves.len().try_into().unwrap()
        } else {
            std::u64::MAX
        };

        Ok(SolutionWithCost {
            solution: self,
            cost,
        })
    }
}

type Cost = u64;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SolutionWithCost {
    pub solution: Solution,
    pub cost: Cost,
}

impl SolutionWithCost {
    pub fn into_inner(self) -> Solution {
        self.solution
    }
}
