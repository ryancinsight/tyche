//! Explicit variance denominator policies.

use eunomia::RealField;

use super::InsufficientSamples;

/// A statically selected variance convention.
pub trait VariancePolicy<T: RealField> {
    /// Minimum observation count.
    const MINIMUM_SAMPLES: u64;

    /// Convert centered sum of squares into variance.
    ///
    /// # Errors
    ///
    /// Returns [`InsufficientSamples`] when the policy's denominator is
    /// undefined.
    fn variance(count: u64, centered_sum: T) -> Result<T, InsufficientSamples>;
}

/// Zero-sized population-variance policy, dividing by `n`.
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
        Ok(centered_sum / T::from_f64(count as f64))
    }
}

/// Zero-sized unbiased sample-variance policy, dividing by `n-1`.
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
        Ok(centered_sum / T::from_f64((count - 1) as f64))
    }
}
