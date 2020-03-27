use crate::common::{random_vector4, random_vector4_near};
use crate::problem::{Domain, Problem, ProblemArgument, ProblemValue};
use nalgebra::Vector4;
use std::ops::RangeInclusive;

pub type Scalar = f64;
fn into_scalar<T: Into<Scalar>>(value: T) -> Scalar {
    value.into()
}

pub struct HappyCatDomain;
const DOMAIN_BOUNDS: RangeInclusive<Scalar> = -2.0..=2.0;

impl Domain for HappyCatDomain {
    type Argument = Vector4<Scalar>;
    type Value = Scalar;

    fn random(scale: f64) -> Self::Argument {
        random_vector4(DOMAIN_BOUNDS, scale)
    }

    fn random_near(point: &Self::Argument, scale: f64) -> Self::Argument {
        random_vector4_near(DOMAIN_BOUNDS, point, scale)
    }
}

pub const NUM_DIMENSIONS: u8 = 4;
pub const ALPHA: Scalar = 0.125;

pub struct HappyCat;

impl Problem for HappyCat {
    type Domain = HappyCatDomain;

    fn value(argument: &ProblemArgument<Self>) -> ProblemValue<Self> {
        let norm2 = argument.norm_squared();
        let first_addend = Scalar::powf(
            Scalar::abs(norm2 - into_scalar(NUM_DIMENSIONS)),
            2.0 * ALPHA,
        );
        let num_dimensions_inv = into_scalar(NUM_DIMENSIONS).recip();
        let second_addend = num_dimensions_inv * (0.5 * norm2 + argument.sum());

        first_addend + second_addend + 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root() {
        assert!(HappyCat::value(Vector4::new(-1.0, -1.0, -1.0, -1.0)).abs() < std::f64::EPSILON);
    }
}
