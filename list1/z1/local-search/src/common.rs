use nalgebra::Vector4;
use rand::{distributions::Uniform, prelude::*};
use std::ops::RangeInclusive;

#[inline(always)]
pub(crate) fn scale_range_inclusive(range: RangeInclusive<f64>, scale: f64) -> RangeInclusive<f64> {
    (scale * *range.start())..=(scale * *range.end())
}

#[inline(always)]
pub(crate) fn length_range_inclusive(range: RangeInclusive<f64>) -> f64 {
    range.end() - range.start()
}

#[inline(always)]
pub(crate) fn random_vector4(domain_bound: RangeInclusive<f64>, scale: f64) -> Vector4<f64> {
    let bound = scale_range_inclusive(domain_bound, scale);
    let dist = Uniform::from(bound);
    Vector4::from_distribution(&dist, &mut thread_rng())
}

#[inline(always)]
pub(crate) fn random_vector4_near(
    domain_bound: RangeInclusive<f64>,
    point: Vector4<f64>,
    scale: f64,
) -> Vector4<f64> {
    let mut rng = thread_rng();
    let distance = scale * length_range_inclusive(domain_bound);
    point.map(|elem| {
        let dist = Uniform::new_inclusive(elem - distance, elem + distance);
        rng.sample(dist)
    })
}
