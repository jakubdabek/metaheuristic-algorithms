use crate::assoc_fcs;
use crate::problem::{Domain, Problem};
use nalgebra::Vector4;
use rand::distributions::Uniform;
use rand::prelude::*;

pub type Scalar = f64;
fn into_scalar<T: Into<Scalar>>(value: T) -> Scalar {
    value.into()
}

pub struct HappyCatDomain;

impl Domain for HappyCatDomain {
    type Argument = Vector4<Scalar>;
    type Value = Scalar;

    fn random(scale: f32) -> Self::Argument {
        let bound = into_scalar(scale * 2.0);
        let dist = Uniform::new_inclusive(-bound, bound);
        Vector4::from_distribution(&dist, &mut thread_rng())
    }
}

pub const NUM_DIMENSIONS: u8 = 4;
pub const ALPHA: Scalar = 0.125;

pub struct HappyCat;

impl Problem for HappyCat {
    type Domain = HappyCatDomain;

    fn value(
        argument: assoc_fcs!(Problem->Domain->Argument),
    ) -> assoc_fcs!(Problem->Domain->Value) {
        let norm2 = argument.norm_squared();
        let first_addend = Scalar::powf(norm2 - into_scalar(NUM_DIMENSIONS), 2.0 * ALPHA);
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
