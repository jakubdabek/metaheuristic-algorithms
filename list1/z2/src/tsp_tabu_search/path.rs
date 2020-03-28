use super::{Cost, CostMatrix, NodeIndex};
use itertools::Itertools;
use rand::prelude::*;
use std::iter::FromIterator;
use std::ops::Index;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub(crate) struct Path {
    nodes: Vec<NodeIndex>,
}

impl Index<usize> for Path {
    type Output = NodeIndex;

    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PathWithCost {
    path: Path,
    cost: Cost,
}

impl Index<usize> for PathWithCost {
    type Output = NodeIndex;

    fn index(&self, index: usize) -> &Self::Output {
        self.path.index(index)
    }
}

impl AsRef<Path> for PathWithCost {
    fn as_ref(&self) -> &Path {
        self.inner()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NeighbourSwapNodes<'a> {
    original: &'a PathWithCost,
    i: NodeIndex,
    j: NodeIndex,
    swapped: Path,
}

impl Path {
    pub fn new_random(size: usize) -> Self {
        let mut path = Vec::from_iter(0..size);
        let mut rng = thread_rng();
        path[1..].shuffle(&mut rng);
        path.push(0);

        Self { nodes: path }
    }
}

impl PathWithCost {
    pub fn cost(&self) -> Cost {
        self.cost
    }

    pub fn inner(&self) -> &Path {
        &self.path
    }

    pub fn into_inner(self) -> Path {
        self.path
    }

    pub fn into_solution(self) -> super::Solution {
        let mut nodes = self.path.nodes;
        for node in nodes.iter_mut() {
            *node += 1;
        }

        super::Solution {
            path: nodes,
            cost: self.cost,
        }
    }

    pub fn from_path(path: Path, costs: &CostMatrix) -> Self {
        let cost = path
            .nodes
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| costs[[a, b]])
            .sum();

        Self { path, cost }
    }

    pub fn neighbour_swap_nodes(&self, i: usize, j: usize) -> NeighbourSwapNodes<'_> {
        NeighbourSwapNodes::from_path_with_cost(self, i, j)
    }
}

impl<'a> NeighbourSwapNodes<'a> {
    pub fn from_path_with_cost(path: &'a PathWithCost, i: usize, j: usize) -> Self {
        debug_assert!(j > i);
        let mut new_path = path.inner().clone();
        new_path.nodes.swap(i, j);

        Self {
            original: path,
            i,
            j,
            swapped: new_path,
        }
    }

    pub fn as_path(&self) -> &Path {
        &self.swapped
    }

    pub fn into_path_with_cost(self, costs: &CostMatrix) -> PathWithCost {
        let Self {
            original,
            i,
            j,
            swapped,
        } = self;

        let (obsolete_cost, new_cost) = if j == i + 1 {
            let cost_consecutive = |a, b, c, d| {
                costs[[original[a], original[b]]]
                    + costs[[original[b], original[c]]]
                    + costs[[original[c], original[d]]]
            };

            let obsolete = cost_consecutive(i - 1, i, j, j + 1);
            let new = cost_consecutive(i - 1, j, i, j + 1);

            (obsolete, new)
        } else {
            let cost_through = |old, new| {
                costs[[original[old - 1], original[new]]]
                    + costs[[original[new], original[old + 1]]]
            };

            let obsolete = cost_through(i, i) + cost_through(j, j);
            let new = cost_through(i, j) + cost_through(j, i);

            (obsolete, new)
        };

        PathWithCost {
            path: swapped,
            cost: original.cost - obsolete_cost + new_cost,
        }
    }
}
