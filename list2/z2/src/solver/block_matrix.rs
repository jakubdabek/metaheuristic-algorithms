use super::{Distance, Value};
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::{SliceInfo, SliceOrIndex};
use rand::{thread_rng, Rng};
use rand_distr::Normal;

const ALLOWED_VALUES: &[Value] = &[0, 32, 64, 128, 160, 192, 223, 255];

#[derive(Debug, Clone)]
pub(super) struct BlockMatrix {
    values: Array2<Value>,
    block_height: usize,
    block_width: usize,
}

fn div_ceil(a: usize, b: usize) -> usize {
    let (div, rem) = (a / b, a % b);

    if rem == 0 {
        div
    } else {
        div + 1
    }
}

fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    assert!(min <= max);
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[inline]
fn iter_step(start: usize, stop: usize, step: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(start), move |&i| Some(i + step)).take_while(move |&i| i < stop)
}

#[inline]
fn iter_step2(start: usize, stop: usize, step: usize) -> impl Iterator<Item = usize> {
    (start..stop).step_by(step)
}

fn iter_signal_last<T, I: Iterator<Item = T>>(iter: I) -> impl Iterator<Item = (bool, T)> {
    let mut iter = iter.peekable();
    std::iter::from_fn(move || {
        if let Some(current) = iter.next() {
            if iter.peek().is_some() {
                Some((false, current))
            } else {
                Some((true, current))
            }
        } else {
            None
        }
    })
}

impl BlockMatrix {
    pub fn zeros(
        block_height: usize,
        block_width: usize,
        outer_height: usize,
        outer_width: usize,
    ) -> Self {
        let h = outer_height / block_height;
        let w = outer_width / block_width;

        Self {
            values: Array2::zeros((h, w)),
            block_height,
            block_width,
        }
    }

    pub fn to_full_size(&self, dim: Ix2) -> Array2<Value> {
        let mut arr = Array2::zeros(dim);

        todo!();

        arr
    }

    pub fn perturb_values(mut self) -> Self {
        let mut rng = thread_rng();
        let normal = Normal::new(0.0_f64, 2.0).unwrap();
        for block in self.values.iter_mut() {
            if rng.gen_bool(0.2) {
                let moved = rng.sample(normal).round() as i8 + (*block as i8);
                *block = clamp(moved, 0, ALLOWED_VALUES.len() as i8 - 1) as u8;
            }
        }

        self
    }

    #[inline]
    fn sum_slice(
        &self,
        i: usize,
        j: usize,
        other: &Array2<Value>,
        slice: &SliceInfo<[SliceOrIndex; 2], Ix2>,
        recip: f64,
    ) -> f64 {
        let block = other.slice(slice);
        let val = ALLOWED_VALUES[self.values[[i, j]] as usize];

        block
            .iter()
            .map(|v| (v - val) as f64)
            .map(|v| v * v * recip)
            .sum()
    }

    pub fn distance_from(&self, other: &Array2<Value>) -> Distance {
        let (h, w) = self.values.dim();
        let (bh, bw) = (self.block_height, self.block_width);
        let (n, m) = other.dim();

        let recip = f64::recip((n * m) as f64);

        let mut distance = 0.0;

        let is = iter_step(0, n, bh);
        for (last_row, i) in iter_signal_last(is) {
            let js = iter_step(0, m, bw);
            for (last_col, j) in iter_signal_last(js) {
                let i_bound = if last_row { n } else { i + bh };
                let j_bound = if last_col { m } else { j + bw };
                distance += self.sum_slice(i, j, other, s![i..i_bound, j..j_bound], recip);
            }
        }

        distance
    }
}
