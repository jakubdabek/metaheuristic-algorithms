use super::direction::Direction;
use crate::board::Board;
use crate::point::Point;
use itertools::Itertools as _;
use rand::distributions::{Standard, Uniform};
use rand::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct Path {
    pub(crate) starting_point: Point,
    pub(crate) ending_point: Point,
    pub(crate) moves: Vec<Direction>,
    pub(crate) cost: Option<u64>,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.starting_point
                .cmp(&other.starting_point)
                .then(self.moves.len().cmp(&other.moves.len()))
                .then_with(|| self.moves.cmp(&other.moves)),
        )
    }
}

fn debug_print_fn(_f: impl FnOnce()) {
    // _f()
}

impl Path {
    pub fn new(starting_point: Point, moves: Vec<Direction>, board: &Board) -> Self {
        let mut path = Self {
            starting_point,
            ending_point: starting_point,
            moves,
            cost: None,
        };

        path.verify(0, board);

        path
    }

    pub fn new_to_exit(board: &Board) -> Self {
        let (h, w) = board.fields.dim();
        let mut path = Path {
            starting_point: board.agent_position,
            ending_point: board.agent_position,
            moves: Vec::with_capacity(h * w),
            cost: None,
        };

        path.extend_to_exit(board);
        path
    }

    #[inline]
    pub fn get_cost(&self) -> u64 {
        self.cost.unwrap_or(std::u64::MAX)
    }

    fn extend_to_exit(&mut self, board: &Board) {
        let (h, w) = board.fields.dim();
        let rng = &mut thread_rng();

        if !self.verify(0, board) {
            loop {
                if self.extend(rng.sample_iter(Standard).take(h + w), board) {
                    break;
                }
            }
        }
    }

    fn verify(&mut self, from_index: usize, board: &Board) -> bool {
        debug_print_fn(|| {
            eprintln!(
                "verify:               {} {:?}",
                self.ending_point, self.moves
            )
        });
        // TODO: check more sanely
        let mut current_pos = if from_index == 0 {
            self.starting_point
        } else {
            self.ending_point
        };
        for (dir, index) in self.moves[from_index..].iter().zip(from_index..) {
            if let Some((exit_dir, ending_point)) = board.move_into_exit(current_pos) {
                self.moves.truncate(index);
                self.moves.push(exit_dir);
                self.ending_point = ending_point;
                self.cost = Some((index + 1) as _);

                debug_print_fn(|| {
                    eprintln!(
                        "found exit:           {} {:?}",
                        self.ending_point, self.moves
                    )
                });

                return true;
            }

            let new_pos = dir.move_point(current_pos);
            if !board.is_valid_position(new_pos) {
                debug_print_fn(|| {
                    eprintln!(
                        "invalid pos at {}:    {}, {} {:?}",
                        index, new_pos, self.ending_point, self.moves
                    )
                });
                self.moves.truncate(index);
                break;
            }

            current_pos = new_pos;
        }

        self.ending_point = current_pos;
        self.cost = None;
        debug_print_fn(|| {
            eprintln!(
                "verified all:          {} {:?}",
                self.ending_point, self.moves
            )
        });
        false
    }

    fn extend(&mut self, new_moves: impl Iterator<Item = Direction>, board: &Board) -> bool {
        let current_length = self.moves.len();
        self.moves.extend(new_moves);
        self.verify(current_length, board)
    }

    pub fn neighbour_by_swap_extend(&self, num_mean: u64, board: &Board) -> Self {
        let mut new = self.clone();

        let rng = &mut thread_rng();
        let dist = Uniform::new(0, self.moves.len());
        let num_dist = rand_distr::Normal::new(num_mean as f64, 4.0).unwrap();

        let num = rng.sample_iter(num_dist).find(|&x| x > 1.0).unwrap() as usize;
        for (a, b) in rng.sample_iter(dist).tuples().take(num) {
            new.moves.swap(a, b);
        }

        new.extend_to_exit(board);
        new
    }

    pub fn remove_redundancies(&mut self) {
        Direction::remove_redundancies(&mut self.moves);
    }
}

impl Path {
    pub fn recombine_splice(
        p1: &Self,
        p2: &Self,
        start1: usize,
        start2: usize,
        end2: usize,
        board: &Board,
    ) -> Self {
        let len = end2 - start2;
        let cap = std::cmp::max(p1.moves.len(), start1 + len);
        let mut moves = Vec::with_capacity(cap);

        moves.extend_from_slice(&p1.moves[..start1]);
        moves.extend_from_slice(&p2.moves[start2..end2]);
        if let Some(s) = p1.moves.get(start1 + len..) {
            moves.extend_from_slice(s);
        }

        Self::new(p1.starting_point, moves, board)
    }

    pub fn mutate_swap(
        path: &Self,
        board: &Board,
        positions: impl Iterator<Item = (usize, usize)>,
    ) -> Self {
        let mut moves = path.moves.clone();
        for (pos1, pos2) in positions {
            moves.swap(pos1, pos2);
        }

        Self::new(path.starting_point, moves, board)
    }
}
