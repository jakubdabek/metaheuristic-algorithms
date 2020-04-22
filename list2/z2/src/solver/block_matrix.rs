use super::{Distance, Value};
use ndarray::prelude::*;
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
fn iter_stop_step(start: usize, stop: usize, step: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(start), move |&i| Some(i + step)).take_while(move |&i| i < stop)
}

#[inline]
fn iter_step(start: usize, step: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(start), move |&i| Some(i + step))
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

macro_rules! process_blocks_decl {
($name:ident [$($mut_:tt)?] $slice:ident => $view:ty) => {
    fn $name<'a, F>(&self, full_size: &'a $($mut_)? Array2<Value>, mut f: F)
    where
        F: FnMut(Value, $view),
    {
        let (h, w) = self.values.dim();
        let (bh, bw) = (self.block_height, self.block_width);
        let (n, m) = full_size.dim();

        let is = iter_step(0, bh).take(h);
        for (i, (last_row, full_i)) in iter_signal_last(is).enumerate() {
            let js = iter_step(0, bw).take(w);
            for (j, (last_col, full_j)) in iter_signal_last(js).enumerate() {
                let i_bound = if last_row { n } else { full_i + bh };
                let j_bound = if last_col { m } else { full_j + bw };

                f(self.values[[i, j]], full_size.$slice(s![full_i..i_bound, full_j..j_bound]));
            }
        }
    }
};
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

        self.process_blocks_mut(&mut arr, |val, mut block| {
            let val = ALLOWED_VALUES[val as usize];
            block.fill(val);
        });

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

    process_blocks_decl!(process_blocks[] slice => ArrayView2<Value>);
    process_blocks_decl!(process_blocks_mut[mut] slice_mut => ArrayViewMut2<Value>);

    pub fn distance_from(&self, other: &Array2<Value>) -> Distance {
        let (n, m) = other.dim();
        let recip = f64::recip((n * m) as f64);

        let mut distance = 0.0_f64;

        self.process_blocks(other, |val, block| {
            let val = ALLOWED_VALUES[val as usize];

            distance += block
                .iter()
                .map(|&v| v as f64 - val as f64)
                .map(|v| v * v * recip)
                .sum::<f64>()
        });

        distance
    }
}
