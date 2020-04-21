use nalgebra::Vector4;
use std::ops::RangeInclusive;
use rand::prelude::*;
use rand::distributions::Uniform;

pub type Scalar = f64;

use std::f64 as scalar;

pub type Argument = Vector4<Scalar>;
pub type Value = f64;


pub const NUM_DIMENSIONS: u8 = 4;

#[inline(always)]
fn length_range_inclusive(range: &RangeInclusive<f64>) -> f64 {
    range.end() - range.start()
}

#[inline(always)]
fn random_vector4_near(
    domain_bound: RangeInclusive<f64>,
    point: &Vector4<f64>,
    scale: f64,
) -> Vector4<f64> {
    let mut rng = thread_rng();
    let distance = scale * length_range_inclusive(&domain_bound);
    point.map(|elem| {
        let dist = Uniform::new_inclusive(
            f64::max(elem - distance, *domain_bound.start()),
            f64::min(elem + distance, *domain_bound.end()),
        );
        rng.sample(dist)
    })
}

pub fn value(argument: &Argument) -> Value {
    let norm = argument.norm();

    1.0 - Scalar::cos(2.0 * scalar::consts::PI * norm) + 0.1 * norm
}

pub const DOMAIN_BOUNDS: RangeInclusive<Scalar> = -100.0..=100.0;

pub fn random_near(point: &Argument, scale: f64) -> Argument {
    random_vector4_near(DOMAIN_BOUNDS, point, scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root() {
        assert!(value(&Vector4::new(0.0, 0.0, 0.0, 0.0)).abs() < scalar::EPSILON);
    }
}
