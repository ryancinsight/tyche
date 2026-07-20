//! Explicit variance denominator policies.

use super::InsufficientSamples;
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
    #[expect(
        clippy::cast_precision_loss,
        reason = "the generic numeric contract represents observation counts in T"
    )]
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

/// Zero-sized sample variance (`n-1` denominator).
#[must_use]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SampleVariance;

impl<T: RealField> VariancePolicy<T> for SampleVariance {
    const MINIMUM_SAMPLES: u64 = 2;
    #[expect(
        clippy::cast_precision_loss,
        reason = "the generic numeric contract represents observation counts in T"
    )]
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
