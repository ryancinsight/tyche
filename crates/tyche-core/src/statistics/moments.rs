//! Welford-Chan online moments.

use super::{InsufficientSamples, VariancePolicy, count_as};
use eunomia::RealField;

/// Online scalar mean and centered sum of squares.
#[must_use]
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
    /// Construct empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tyche_core::statistics::Moments;
    ///
    /// let mut m = Moments::<f64>::new();
    /// m.update(1.0);
    /// m.update(2.0);
    /// m.update(3.0);
    /// assert_eq!(m.mean().unwrap(), 2.0);
    /// ```
    pub fn new() -> Self {
        Self {
            count: 0,
            mean: T::ZERO,
            centered_sum: T::ZERO,
        }
    }
    /// Observation count.
    #[must_use]
    pub const fn count(self) -> u64 {
        self.count
    }
    /// Whether empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.count == 0
    }
    /// Add an observation.
    pub fn update(&mut self, value: T) {
        self.count += 1;
        let count = count_as(self.count);
        let delta = value - self.mean;
        self.mean += delta / count;
        self.centered_sum += delta * (value - self.mean);
    }
    /// Merge another accumulator using Chan's recurrence.
    pub fn merge(&mut self, other: Self) {
        if other.count == 0 {
            return;
        }
        if self.count == 0 {
            *self = other;
            return;
        }
        let combined = self.count + other.count;
        let left = count_as::<T>(self.count);
        let right = count_as::<T>(other.count);
        let total = count_as::<T>(combined);
        let delta = other.mean - self.mean;
        self.mean += delta * (right / total);
        self.centered_sum += other.centered_sum + delta * delta * (left * right / total);
        self.count = combined;
    }
    /// Arithmetic mean.
    ///
    /// # Errors
    ///
    /// Rejects empty input.
    pub fn mean(self) -> Result<T, InsufficientSamples> {
        if self.count == 0 {
            Err(InsufficientSamples::new(1, 0))
        } else {
            Ok(self.mean)
        }
    }
    /// Centered sum of squares.
    #[must_use]
    pub const fn centered_sum(self) -> T {
        self.centered_sum
    }
    /// Variance under an explicit policy.
    ///
    /// # Errors
    ///
    /// Rejects an undefined denominator.
    pub fn variance<P: VariancePolicy<T>>(self) -> Result<T, InsufficientSamples> {
        P::variance(self.count, self.centered_sum)
    }
}
