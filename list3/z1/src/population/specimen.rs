use super::{Argument, Scalar, SIZE};

pub const GENOME_LENGTH: i64 = 40;
pub const GENOME_MASK: i64 = (1 << GENOME_LENGTH) - 1;
pub const MAX_GENOME_VALUE: i64 = (1 << GENOME_LENGTH) - 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Specimen {
    pub values: [i64; SIZE],
}

impl Specimen {
    pub fn new(values: [i64; SIZE]) -> Self {
        Self { values }
    }

    pub fn from_argument(argument: &Argument) -> Self {
        let mut values = [0; SIZE];
        values.iter_mut().zip(argument.iter()).for_each(|(v, a)| {
            *v = ((a * MAX_GENOME_VALUE as Scalar) as i64 + 5 * MAX_GENOME_VALUE) / 10;
        });

        Self { values }
    }

    pub fn to_argument(&self) -> Argument {
        let mut argument: Argument = [0.0; SIZE];
        argument
            .iter_mut()
            .zip(self.values.iter())
            .for_each(|(a, v)| {
                *a = (v * 10 - 5 * MAX_GENOME_VALUE) as Scalar / MAX_GENOME_VALUE as Scalar;
            });

        argument
    }
}

impl Specimen {
    pub fn recombine_inner(s1: &Self, s2: &Self, i: i64, j: i64) -> Self {
        let len = j - i + 1;
        let mask = ((1 << len) - 1) << (GENOME_LENGTH - j - 1);

        let mut values = [0; SIZE];

        values
            .iter_mut()
            .zip(s1.values.iter().zip(s2.values.iter()))
            .for_each(|(s, (s1, s2))| {
                *s = (s1 & !mask) | (s2 & mask);
                *s &= GENOME_MASK;
            });

        Self { values }
    }

    pub fn recombine_outer(s1: &Self, s2: &Self, i: usize) -> Self {
        let mut values = s1.values;
        values[i] = s2.values[i];

        Self {
            values,
        }
    }

    pub fn mutate_big(specimen: &Self, i: usize) -> Self {
        let mut values = specimen.values;
        let value = &mut values[i];

        *value = !*value & GENOME_MASK;

        Self {
            values,
        }
    }

    pub fn mutate_small(specimen: &Self, positions: &[i64]) -> Self {
        let mut values = specimen.values;

        for value in &mut values {
            for &pos in positions {
                *value ^= 1 << pos;
            }
            *value &= GENOME_MASK;
        }

        Self { values }
    }
}
