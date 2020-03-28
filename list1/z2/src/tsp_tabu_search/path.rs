use super::{Cost, CostMatrix, NodeIndex};
use itertools::Itertools;
use rand::prelude::*;
use std::iter::FromIterator;
use std::ops::Index;

#[derive(Debug, Clone)]
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
    pub fn inner(&self) -> &Path {
        &self.path
    }

    pub fn into_inner(self) -> Path {
        self.path
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
        let mut new_path = self.inner().clone();
        new_path.nodes.swap(i, j);

        NeighbourSwapNodes {
            original: self,
            i,
            j,
            swapped: new_path,
        }
    }
}

impl NeighbourSwapNodes<'_> {
    pub fn into_path_with_cost(self, costs: &CostMatrix) -> PathWithCost {
        let Self {
            original,
            i,
            j,
            swapped,
        } = self;

        let cost_from_to = |x, y| {
            costs[[original[i - 1], original[x]]]
                + costs[[original[x], original[i + 1]]]
                + costs[[original[j - 1], original[y]]]
                + costs[[original[y], original[j + 1]]]
        };

        let obsolete_cost = cost_from_to(i, j);
        let new_cost = cost_from_to(j, i);

        PathWithCost {
            path: swapped,
            cost: original.cost - obsolete_cost + new_cost,
        }
    }
}
