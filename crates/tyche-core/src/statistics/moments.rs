//! Welford-Chan online moments.

use eunomia::RealField;

use super::{InsufficientSamples, VariancePolicy};

/// Online scalar mean and centered sum of squares.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Moments<T> {
    count: u64,
    mean: T,
    centered_sum: T,
}

impl<T: RealField> Default for Moments<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RealField> Moments<T> {
    /// Construct an empty accumulator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            count: 0,
            mean: T::ZERO,
            centered_sum: T::ZERO,
        }
    }

    /// Number of observations.
    #[must_use]
    pub const fn count(self) -> u64 {
        self.count
    }

    /// Whether no observation has been accumulated.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.count == 0
    }

    /// Add one observation using Welford's recurrence.
    pub fn update(&mut self, value: T) {
        self.count += 1;
        let count = T::from_f64(self.count as f64);
        let delta = value - self.mean;
        self.mean += delta / count;
        let delta_after = value - self.mean;
        self.centered_sum += delta * delta_after;
    }

    /// Merge another accumulator using Chan's pairwise recurrence.
    pub fn merge(&mut self, other: Self) {
        if other.count == 0 {
            return;
        }
        if self.count == 0 {
            *self = other;
            return;
        }
        let combined = self.count + other.count;
        let left_count = T::from_f64(self.count as f64);
        let right_count = T::from_f64(other.count as f64);
        let combined_count = T::from_f64(combined as f64);
        let delta = other.mean - self.mean;
        self.mean += delta * (right_count / combined_count);
        self.centered_sum +=
            other.centered_sum + delta * delta * (left_count * right_count / combined_count);
        self.count = combined;
    }

    /// Arithmetic mean.
    ///
    /// # Errors
    ///
    /// Returns [`InsufficientSamples`] when empty.
    pub fn mean(self) -> Result<T, InsufficientSamples> {
        if self.count == 0 {
            Err(InsufficientSamples::new(1, 0))
        } else {
            Ok(self.mean)
        }
    }

    /// Centered sum of squares `Σ(xᵢ - mean)²`.
    #[must_use]
    pub const fn centered_sum(self) -> T {
        self.centered_sum
    }

    /// Variance under an explicit zero-sized denominator policy.
    ///
    /// # Errors
    ///
    /// Returns [`InsufficientSamples`] when the selected convention is
    /// undefined.
    pub fn variance<P: VariancePolicy<T>>(self) -> Result<T, InsufficientSamples> {
        P::variance(self.count, self.centered_sum)
    }
}
