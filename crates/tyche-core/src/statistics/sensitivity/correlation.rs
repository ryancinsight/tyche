//! Correlation-based sensitivity screening.

use eunomia::RealField;

use super::estimator::OnlineEstimator;
use super::private::Sealed;
use super::report::SensitivityReport;
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

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize> Sealed
    for CorrelationScreening<T, PARAMETERS, OUTPUTS>
{
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
    pub fn update_outputs(&mut self, parameters: &[T; PARAMETERS], responses: &[T; OUTPUTS]) {
        OnlineEstimator::accumulate(self, (parameters, responses));
    }

    /// Produce squared Pearson indices.
    ///
    /// # Errors
    ///
    /// Requires two observations.
    pub fn report(self) -> Result<SensitivityReport<T, PARAMETERS, OUTPUTS>, InsufficientSamples> {
        OnlineEstimator::report(self)
    }
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    OnlineEstimator<T, PARAMETERS, OUTPUTS> for CorrelationScreening<T, PARAMETERS, OUTPUTS>
{
    type Report = SensitivityReport<T, PARAMETERS, OUTPUTS>;
    type Observation<'a> = (&'a [T; PARAMETERS], &'a [T; OUTPUTS]);
    const MINIMUM_SAMPLES: u64 = 2;

    fn sample_count(&self) -> u64 {
        self.count
    }

    fn accumulate(&mut self, (parameters, responses): Self::Observation<'_>) {
        self.count += 1;
        let count = self.count_as();
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

    fn finish(self) -> Self::Report {
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
        SensitivityReport::from_correlations(self.count, values)
    }
}

impl<T: RealField, const PARAMETERS: usize> CorrelationScreening<T, PARAMETERS, 1> {
    /// Add a scalar response to the single-output estimator.
    pub fn update(&mut self, parameters: &[T; PARAMETERS], response: T) {
        self.update_outputs(parameters, &[response]);
    }
}
