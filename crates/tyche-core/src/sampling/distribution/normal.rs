//! Standard-normal inverse transform.

use core::marker::PhantomData;

use crate::sampling::counter::{
    Counter, SampleScalar, Seed, StandardNormalAngle, StandardNormalRadius, StreamAlgorithm,
};

/// Zero-sized standard-normal counter sampler over scalar type `T`.
///
/// Box-Muller consumes two independently domain-separated counter words. No
/// mutable RNG state or cache is shared between trials.
#[must_use]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StandardNormal<T, A> {
    _policy: PhantomData<(T, A)>,
}

impl<T: SampleScalar, A: StreamAlgorithm> StandardNormal<T, A> {
    /// Return one standard-normal variate addressed by `(seed, sample, draw)`.
    ///
    /// The Box-Muller transform derives two independent uniform variates from
    /// typed counter domains, converts them to `T`, and applies the
    /// closed-form polar transformation at the precision of `T`.
    #[must_use]
    pub fn at(seed: Seed, sample: u64, draw: u64) -> T {
        use eunomia::{FloatElement, NumericElement};

        let radius_unit = Counter::<StandardNormalRadius, A>::open_unit(seed, sample, draw);
        let angle_unit = Counter::<StandardNormalAngle, A>::unit(seed, sample, draw);
        let radius = <T as NumericElement>::sqrt(
            <T as FloatElement>::from_f64(-2.0) * <T as FloatElement>::ln(radius_unit),
        );
        radius * <T as FloatElement>::cos(<T as eunomia::RealField>::TAU * angle_unit)
    }
}
