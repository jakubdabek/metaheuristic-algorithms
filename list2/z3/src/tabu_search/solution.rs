use super::direction::Direction;
use crate::board::Board;
use crate::point::Point;
use itertools::Itertools as _;
use rand::distributions::Uniform;
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPath {
    starting_point: Point,
    ending_point: Point,
    moves: Vec<Direction>,
}

impl PartialPath {
    pub fn verify(starting_point: Point, mut moves: Vec<Direction>, board: &Board) -> (Self, bool) {
        debug_assert!(
            board.is_valid_position(starting_point),
            "starting point must be valid!"
        );

        let mut current_pos = starting_point;
        for (count, &dir) in moves.iter().enumerate() {
            if let Some((exit_dir, ending_point)) = board.move_into_exit(current_pos) {
                moves.truncate(count);
                moves.push(exit_dir);

                return (
                    Self {
                        starting_point,
                        ending_point,
                        moves,
                    },
                    true,
                );
            }

            let new_pos = dir.move_point(current_pos);
            if !board.is_valid_position(new_pos) {
                moves.truncate(count);
                break;
            }

            current_pos = new_pos;
        }

        (
            Self {
                starting_point,
                ending_point: current_pos,
                moves,
            },
            false,
        )
    }
}

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
        let mut moves = self.moves;
        let mut i = 0;
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


    pub fn into_solution_with_cost(self, board: &Board) -> Result<SolutionWithCost, Solution> {
        todo!("remove")
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
