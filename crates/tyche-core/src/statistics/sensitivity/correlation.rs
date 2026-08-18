//! Correlation-based sensitivity screening.

use eunomia::RealField;

use crate::statistics::InsufficientSamples;

/// Online parameter-response correlation screening.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CorrelationScreening<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> {
    count: u64,
    mean_parameters: [T; PARAMETERS],
    mean_response: [T; OUTPUTS],
    parameter_sums: [T; PARAMETERS],
    response_sum: [T; OUTPUTS],
    co_moments: [[T; PARAMETERS]; OUTPUTS],
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize> Default
    for CorrelationScreening<T, PARAMETERS, OUTPUTS>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    CorrelationScreening<T, PARAMETERS, OUTPUTS>
{
    /// Construct empty.
    pub fn new() -> Self {
        Self {
            count: 0,
            mean_parameters: [T::ZERO; PARAMETERS],
            mean_response: [T::ZERO; OUTPUTS],
            parameter_sums: [T::ZERO; PARAMETERS],
            response_sum: [T::ZERO; OUTPUTS],
            co_moments: [[T::ZERO; PARAMETERS]; OUTPUTS],
        }
    }
    /// Add one parameter vector and its output vector.
    ///
    /// The output dimension is a const generic so one estimator can retain
    /// independent correlation statistics for every model output without
    /// allocating per observation.
    #[expect(
        clippy::cast_precision_loss,
        reason = "the generic numeric contract represents observation counts in T"
    )]
    pub fn update_outputs(&mut self, parameters: &[T; PARAMETERS], responses: &[T; OUTPUTS]) {
        self.count += 1;
        let count = T::from_f64(self.count as f64);
        let mut response_delta = [T::ZERO; OUTPUTS];
        let mut response_after = [T::ZERO; OUTPUTS];
        for (((mean, delta), after), &response) in self
            .mean_response
            .iter_mut()
            .zip(response_delta.iter_mut())
            .zip(response_after.iter_mut())
            .zip(responses.iter())
        {
            *delta = response - *mean;
            *mean += *delta / count;
            *after = response - *mean;
        }
        for ((sum, &delta), &after) in self
            .response_sum
            .iter_mut()
            .zip(response_delta.iter())
            .zip(response_after.iter())
        {
            *sum += delta * after;
        }
        let mut parameter_deltas = [T::ZERO; PARAMETERS];
        let mut parameter_after = [T::ZERO; PARAMETERS];
        for (((mean, delta), after), &parameter) in self
            .mean_parameters
            .iter_mut()
            .zip(parameter_deltas.iter_mut())
            .zip(parameter_after.iter_mut())
            .zip(parameters.iter())
        {
            *delta = parameter - *mean;
            *mean += *delta / count;
            *after = parameter - *mean;
        }
        for ((sum, &delta), &after) in self
            .parameter_sums
            .iter_mut()
            .zip(parameter_deltas.iter())
            .zip(parameter_after.iter())
        {
            *sum += delta * after;
        }
        for (co_moments, &after) in self.co_moments.iter_mut().zip(response_after.iter()) {
            for (co_moment, &delta) in co_moments.iter_mut().zip(parameter_deltas.iter()) {
                *co_moment += delta * after;
            }
        }
    }

    /// Produce squared Pearson indices.
    ///
    /// # Errors
    ///
    /// Requires two observations.
    pub fn report(self) -> Result<SensitivityReport<T, PARAMETERS, OUTPUTS>, InsufficientSamples> {
        if self.count < 2 {
            return Err(InsufficientSamples::new(2, self.count));
        }
        let mut values = [[T::ZERO; PARAMETERS]; OUTPUTS];
        for (value_row, (co_moments, &response_sum)) in values
            .iter_mut()
            .zip(self.co_moments.iter().zip(self.response_sum.iter()))
        {
            for ((value, &co_moment), &parameter_sum) in value_row
                .iter_mut()
                .zip(co_moments.iter())
                .zip(self.parameter_sums.iter())
            {
                let denominator = parameter_sum * response_sum;
                if denominator > T::ZERO {
                    let raw = co_moment * co_moment / denominator;
                    *value = raw.clamp(T::ZERO, T::ONE);
                }
            }
        }
        Ok(SensitivityReport {
            sample_count: self.count,
            squared_correlations: values,
        })
    }
}

/// Correlation-based screening report.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensitivityReport<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> {
    sample_count: u64,
    squared_correlations: [[T; PARAMETERS]; OUTPUTS],
}

impl<T, const PARAMETERS: usize, const OUTPUTS: usize> SensitivityReport<T, PARAMETERS, OUTPUTS> {
    /// Sample count.
    #[must_use]
    pub const fn sample_count(&self) -> u64 {
        self.sample_count
    }
    /// Borrow indices.
    #[must_use]
    pub const fn squared_correlations_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        &self.squared_correlations
    }
}

impl<T, const PARAMETERS: usize> SensitivityReport<T, PARAMETERS, 1> {
    /// Borrow the single-output squared Pearson indices.
    #[must_use]
    pub const fn squared_correlations(&self) -> &[T; PARAMETERS] {
        &self.squared_correlations[0]
    }
}

impl<T: RealField, const PARAMETERS: usize> CorrelationScreening<T, PARAMETERS, 1> {
    /// Add a scalar response to the single-output estimator.
    pub fn update(&mut self, parameters: &[T; PARAMETERS], response: T) {
        self.update_outputs(parameters, &[response]);
    }
}
