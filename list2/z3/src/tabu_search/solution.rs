use super::direction::Direction;
use crate::board::Board;
use itertools::Itertools as _;
use rand::distributions::Uniform;
use rand::prelude::*;
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
            let num = rng.sample_iter(num_dist).find(|&x| x > 1.0).unwrap() as usize;
            for (a, b) in (&mut rng).sample_iter(dist).tuples().take(num) {
                new_path.swap(a, b);
            }

            Solution { moves: new_path }
        })
    }

    pub fn remove_redundancies(self) -> Self {
        let mut i = 0;
        let mut moves = self.moves;
        while i + 2 < moves.len() {
            if moves[i].inverse() == moves[i + 1] {
                drop(moves.drain(i..=(i + 1)));
            } else if moves[i].inverse() == moves[i + 2] {
                moves.swap(i + 1, i + 2);
                drop(moves.drain(i..=(i + 1)));
            } else {
                i += 1;
            }
        }

        Self { moves }
    }

    pub fn into_solution_with_cost(mut self, board: &Board) -> Result<SolutionWithCost, Solution> {
        let mut current_pos = board.agent_position;
        for (count, &dir) in self.moves.iter().enumerate() {
            if let Some((goal_dir, _)) = board
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
