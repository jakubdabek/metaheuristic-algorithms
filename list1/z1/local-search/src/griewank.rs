use crate::Problem;
use nalgebra::Vector4;
use std::ops::Mul;

pub type Domain = f32;
fn into_domain<T: Into<Domain>>(value: T) -> Domain {
    value.into()
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
    type Domain = Vector4<Domain>;
    type CoDomain = Domain;

    fn value(argument: Self::Domain) -> Self::CoDomain {
        let norm2 = argument.norm_squared(); // sum of squares of components
        let minuend = norm2 / 4000.0;
        let inv_squares = (1..=NUM_DIMENSIONS)
            .map(into_domain)
            .map(Domain::sqrt)
            .map(Domain::recip);
        let subtrahend = argument
            .iter()
            .copied()
            .zip(inv_squares)
            .map(uncurry(Domain::mul))
            .map(Domain::cos)
            .product::<Domain>();

        1.0 + minuend - subtrahend
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root() {
        assert!(Griewank::value(Vector4::new(0.0, 0.0, 0.0, 0.0)).abs() < std::f32::EPSILON);
    }
}
