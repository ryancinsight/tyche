//! Explicit variance denominator policies.

use super::{InsufficientSamples, count_as};
use eunomia::RealField;

/// A statically selected variance convention.
pub trait VariancePolicy<T: RealField> {
    /// Minimum observation count.
    const MINIMUM_SAMPLES: u64;
    /// Convert centered sum into variance.
    ///
    /// # Errors
    ///
    /// Rejects an undefined denominator.
    fn variance(count: u64, centered_sum: T) -> Result<T, InsufficientSamples>;
}

/// Zero-sized population variance (`n` denominator).
#[must_use]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PopulationVariance;

impl<T: RealField> VariancePolicy<T> for PopulationVariance {
    const MINIMUM_SAMPLES: u64 = 1;
    fn variance(count: u64, centered_sum: T) -> Result<T, InsufficientSamples> {
        if count == 0 {
            return Err(InsufficientSamples::new(
                <Self as VariancePolicy<T>>::MINIMUM_SAMPLES,
                count,
            ));
        }
        Ok(centered_sum / count_as(count))
    }
}

/// Zero-sized sample variance (`n-1` denominator).
#[must_use]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SampleVariance;

impl<T: RealField> VariancePolicy<T> for SampleVariance {
    const MINIMUM_SAMPLES: u64 = 2;
    fn variance(count: u64, centered_sum: T) -> Result<T, InsufficientSamples> {
        if count < <Self as VariancePolicy<T>>::MINIMUM_SAMPLES {
            return Err(InsufficientSamples::new(
                <Self as VariancePolicy<T>>::MINIMUM_SAMPLES,
                count,
            ));
        }
        Ok(centered_sum / count_as(count - 1))
    }
}
