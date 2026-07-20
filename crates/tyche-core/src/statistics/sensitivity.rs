//! Squared-correlation global screening.

use eunomia::RealField;

use super::InsufficientSamples;

/// Online input-response correlation screening for `PARAMETERS` dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CorrelationScreening<T, const PARAMETERS: usize> {
    count: u64,
    mean_parameters: [T; PARAMETERS],
    mean_response: T,
    parameter_centered_sums: [T; PARAMETERS],
    response_centered_sum: T,
    co_moments: [T; PARAMETERS],
}

impl<T: RealField, const PARAMETERS: usize> Default for CorrelationScreening<T, PARAMETERS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RealField, const PARAMETERS: usize> CorrelationScreening<T, PARAMETERS> {
    /// Construct an empty screening accumulator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            count: 0,
            mean_parameters: [T::ZERO; PARAMETERS],
            mean_response: T::ZERO,
            parameter_centered_sums: [T::ZERO; PARAMETERS],
            response_centered_sum: T::ZERO,
            co_moments: [T::ZERO; PARAMETERS],
        }
    }

    /// Add one parameter-response pair.
    pub fn update(&mut self, parameters: &[T; PARAMETERS], response: T) {
        self.count += 1;
        let count = T::from_f64(self.count as f64);
        let response_delta = response - self.mean_response;
        self.mean_response += response_delta / count;
        let response_after = response - self.mean_response;
        self.response_centered_sum += response_delta * response_after;

        for dimension in 0..PARAMETERS {
            let parameter_delta = parameters[dimension] - self.mean_parameters[dimension];
            self.mean_parameters[dimension] += parameter_delta / count;
            let parameter_after = parameters[dimension] - self.mean_parameters[dimension];
            self.parameter_centered_sums[dimension] += parameter_delta * parameter_after;
            self.co_moments[dimension] += parameter_delta * response_after;
        }
    }

    /// Produce squared Pearson correlation indices.
    ///
    /// This is a screening measure, not a Sobol variance decomposition. It is
    /// named accordingly so first-order and total-effect indices are never
    /// falsely implied.
    ///
    /// # Errors
    ///
    /// Returns [`InsufficientSamples`] for fewer than two observations.
    pub fn report(self) -> Result<SensitivityReport<T, PARAMETERS>, InsufficientSamples> {
        if self.count < 2 {
            return Err(InsufficientSamples::new(2, self.count));
        }
        let mut squared_correlations = [T::ZERO; PARAMETERS];
        for (dimension, output) in squared_correlations.iter_mut().enumerate() {
            let denominator = self.parameter_centered_sums[dimension] * self.response_centered_sum;
            if denominator > T::ZERO {
                let raw = self.co_moments[dimension] * self.co_moments[dimension] / denominator;
                *output = raw.clamp(T::ZERO, T::ONE);
            }
        }
        Ok(SensitivityReport {
            sample_count: self.count,
            squared_correlations,
        })
    }
}

/// Correlation-based parameter screening report.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensitivityReport<T, const PARAMETERS: usize> {
    sample_count: u64,
    squared_correlations: [T; PARAMETERS],
}

impl<T, const PARAMETERS: usize> SensitivityReport<T, PARAMETERS> {
    /// Number of observations used.
    #[must_use]
    pub const fn sample_count(&self) -> u64 {
        self.sample_count
    }

    /// Borrow squared Pearson correlation values in parameter order.
    #[must_use]
    pub const fn squared_correlations(&self) -> &[T; PARAMETERS] {
        &self.squared_correlations
    }
}
