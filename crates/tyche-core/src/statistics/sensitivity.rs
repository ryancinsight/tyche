//! Squared-correlation global screening.

use super::InsufficientSamples;
use eunomia::RealField;

/// Online parameter-response correlation screening.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CorrelationScreening<T, const PARAMETERS: usize> {
    count: u64,
    mean_parameters: [T; PARAMETERS],
    mean_response: T,
    parameter_sums: [T; PARAMETERS],
    response_sum: T,
    co_moments: [T; PARAMETERS],
}

impl<T: RealField, const PARAMETERS: usize> Default for CorrelationScreening<T, PARAMETERS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RealField, const PARAMETERS: usize> CorrelationScreening<T, PARAMETERS> {
    /// Construct empty.
    #[must_use]
    pub fn new() -> Self {
        Self {
            count: 0,
            mean_parameters: [T::ZERO; PARAMETERS],
            mean_response: T::ZERO,
            parameter_sums: [T::ZERO; PARAMETERS],
            response_sum: T::ZERO,
            co_moments: [T::ZERO; PARAMETERS],
        }
    }
    /// Add a pair.
    #[expect(
        clippy::cast_precision_loss,
        reason = "the generic numeric contract represents observation counts in T"
    )]
    pub fn update(&mut self, parameters: &[T; PARAMETERS], response: T) {
        self.count += 1;
        let count = T::from_f64(self.count as f64);
        let response_delta = response - self.mean_response;
        self.mean_response += response_delta / count;
        let response_after = response - self.mean_response;
        self.response_sum += response_delta * response_after;
        for (dimension, &parameter) in parameters.iter().enumerate() {
            let delta = parameter - self.mean_parameters[dimension];
            self.mean_parameters[dimension] += delta / count;
            let after = parameter - self.mean_parameters[dimension];
            self.parameter_sums[dimension] += delta * after;
            self.co_moments[dimension] += delta * response_after;
        }
    }
    /// Produce squared Pearson indices.
    ///
    /// # Errors
    ///
    /// Requires two observations.
    pub fn report(self) -> Result<SensitivityReport<T, PARAMETERS>, InsufficientSamples> {
        if self.count < 2 {
            return Err(InsufficientSamples::new(2, self.count));
        }
        let mut values = [T::ZERO; PARAMETERS];
        for (dimension, output) in values.iter_mut().enumerate() {
            let denominator = self.parameter_sums[dimension] * self.response_sum;
            if denominator > T::ZERO {
                let raw = self.co_moments[dimension] * self.co_moments[dimension] / denominator;
                *output = raw.clamp(T::ZERO, T::ONE);
            }
        }
        Ok(SensitivityReport {
            sample_count: self.count,
            squared_correlations: values,
        })
    }
}

/// Correlation-based screening report.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensitivityReport<T, const PARAMETERS: usize> {
    sample_count: u64,
    squared_correlations: [T; PARAMETERS],
}

impl<T, const PARAMETERS: usize> SensitivityReport<T, PARAMETERS> {
    /// Sample count.
    #[must_use]
    pub const fn sample_count(&self) -> u64 {
        self.sample_count
    }
    /// Borrow indices.
    #[must_use]
    pub const fn squared_correlations(&self) -> &[T; PARAMETERS] {
        &self.squared_correlations
    }
}
