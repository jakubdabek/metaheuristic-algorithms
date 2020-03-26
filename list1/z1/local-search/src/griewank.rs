use crate::assoc_fcs;
use crate::problem::{Domain, Problem};
use nalgebra::Vector4;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::ops::Mul;

pub type Scalar = f64;
fn into_scalar<T: Into<Scalar>>(value: T) -> Scalar {
    value.into()
}

pub struct GriewankDomain;

impl Domain for GriewankDomain {
    type Argument = Vector4<Scalar>;
    type Value = Scalar;

    fn random(scale: f32) -> Self::Argument {
        let bound = into_scalar(scale * 600.0);
        let dist = Uniform::new_inclusive(-bound, bound);
        Vector4::from_distribution(&dist, &mut thread_rng())
    }
}

pub const NUM_DIMENSIONS: u8 = 4;

pub struct Griewank;

fn uncurry<F, T1, T2, R>(func: F) -> impl Fn((T1, T2)) -> R
where
    F: Fn(T1, T2) -> R,
{
    move |(a, b)| func(a, b)
}

impl Problem for Griewank {
    type Domain = GriewankDomain;

    fn value(
        argument: assoc_fcs!(Problem->Domain->Argument),
    ) -> assoc_fcs!(Problem->Domain->Value) {
        let norm2 = argument.norm_squared(); // sum of squares of components
        let minuend = norm2 / 4000.0;
        let inv_squares = (1..=NUM_DIMENSIONS)
            .map(into_scalar)
            .map(Scalar::sqrt)
            .map(Scalar::recip);
        let subtrahend = argument
            .iter()
            .copied()
            .zip(inv_squares)
            .map(uncurry(Scalar::mul))
            .map(Scalar::cos)
            .product::<Scalar>();

        1.0 + minuend - subtrahend
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root() {
        assert!(Griewank::value(Vector4::new(0.0, 0.0, 0.0, 0.0)).abs() < std::f64::EPSILON);
    }
}
