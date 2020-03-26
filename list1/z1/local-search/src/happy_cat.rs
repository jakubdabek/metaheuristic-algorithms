use crate::Problem;
use nalgebra::Vector4;

pub type Domain = f32;
fn into_domain<T: Into<Domain>>(value: T) -> Domain {
    value.into()
}

pub const NUM_DIMENSIONS: u8 = 4;
pub const ALPHA: Domain = 0.125;

pub struct HappyCat;

impl Problem for HappyCat {
    type Domain = Vector4<Domain>;
    type CoDomain = Domain;

    fn value(argument: Self::Domain) -> Self::CoDomain {
        let norm2 = argument.norm_squared();
        let first_addend = Domain::powf(norm2 - into_domain(NUM_DIMENSIONS), 2.0 * ALPHA);
        let num_dimensions_inv = into_domain(NUM_DIMENSIONS).recip();
        let second_addend = num_dimensions_inv * (0.5 * norm2 + argument.sum());

        first_addend + second_addend + 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root() {
        assert!(HappyCat::value(Vector4::new(-1.0, -1.0, -1.0, -1.0)).abs() < std::f32::EPSILON);
    }
}
