//! Global sensitivity screening: squared correlation, Morris elementary
//! effects, and Saltelli first- and total-order Sobol' indices.

use core::fmt;

use super::InsufficientSamples;
use eunomia::RealField;

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

/// Morris trajectory elementary-effect batch.
///
/// A Morris trajectory walks `PARAMETERS + 1` design points: a shared start
/// point followed by one step per parameter. Each step changes exactly one
/// coordinate by `+delta` or `-delta`, and the response difference divided by
/// the step size is that parameter's elementary effect. This type validates
/// the trajectory contract and reduces one trajectory to its effect vector.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementaryEffects<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> {
    effects: [[T; PARAMETERS]; OUTPUTS],
}

/// Elementary-effect trajectory construction failures.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementaryEffectsError {
    /// A perturbation step named a parameter outside `0..PARAMETERS`.
    OutOfRangeParameter {
        /// Step position within the trajectory.
        step: usize,
        /// Parameter index supplied for that step.
        index: usize,
    },
    /// The same parameter was perturbed more than once in one trajectory.
    DuplicateParameter {
        /// Repeated parameter index.
        parameter: usize,
    },
    /// The step size must be strictly positive.
    NonPositiveStep,
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    ElementaryEffects<T, PARAMETERS, OUTPUTS>
{
    /// Reduce a validated trajectory to per-parameter, per-output effects.
    ///
    /// `perturbed[step]` is the parameter index changed at each of the
    /// `PARAMETERS` steps and must name every parameter exactly once;
    /// `start_responses` is the model output vector at the trajectory start,
    /// and `step_responses[step]` is the output vector immediately after the
    /// step that perturbed `perturbed[step]`. `delta` is the perturbation
    /// magnitude used for every step.
    ///
    /// # Errors
    ///
    /// Returns [`ElementaryEffectsError`] when the step size is not strictly
    /// positive, a step names an out-of-range parameter, or a parameter is
    /// perturbed more than once.
    ///
    /// # Examples
    ///
    /// ```
    /// use tyche_core::statistics::ElementaryEffects;
    ///
    /// // y = x0 + x1, start (0, 0), perturb x1 then x0 by delta = 0.25.
    /// let effects = ElementaryEffects::<f64, 2>::from_steps(
    ///     &[1, 0],
    ///     0.0,
    ///     &[0.25, 0.5],
    ///     0.25,
    /// )
    /// .expect("valid trajectory");
    /// assert_eq!(effects.effects(), &[1.0, 1.0]);
    /// ```
    pub fn from_steps_outputs(
        perturbed: &[usize; PARAMETERS],
        start_responses: &[T; OUTPUTS],
        step_responses: &[[T; OUTPUTS]; PARAMETERS],
        delta: T,
    ) -> Result<Self, ElementaryEffectsError> {
        if !delta.is_finite() || delta <= T::ZERO {
            return Err(ElementaryEffectsError::NonPositiveStep);
        }
        let mut seen = [false; PARAMETERS];
        let mut effects = [[T::ZERO; PARAMETERS]; OUTPUTS];
        for (step, &parameter) in perturbed.iter().enumerate() {
            if parameter >= PARAMETERS {
                return Err(ElementaryEffectsError::OutOfRangeParameter {
                    step,
                    index: parameter,
                });
            }
            if seen[parameter] {
                return Err(ElementaryEffectsError::DuplicateParameter { parameter });
            }
            seen[parameter] = true;
            for output in 0..OUTPUTS {
                let previous = if step == 0 {
                    start_responses[output]
                } else {
                    step_responses[step - 1][output]
                };
                effects[output][parameter] = (step_responses[step][output] - previous) / delta;
            }
        }
        Ok(Self { effects })
    }

    /// Borrow the per-parameter, per-output elementary effects.
    #[must_use]
    pub const fn effects_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        &self.effects
    }
}

impl<T: RealField, const PARAMETERS: usize> ElementaryEffects<T, PARAMETERS, 1> {
    /// Reduce a scalar-response trajectory to elementary effects.
    ///
    /// `perturbed[step]` must name every parameter exactly once.
    ///
    /// # Errors
    ///
    /// Returns [`ElementaryEffectsError`] when the trajectory is malformed.
    pub fn from_steps(
        perturbed: &[usize; PARAMETERS],
        start_response: T,
        step_responses: &[T; PARAMETERS],
        delta: T,
    ) -> Result<Self, ElementaryEffectsError> {
        let step_responses = core::array::from_fn(|step| [step_responses[step]]);
        Self::from_steps_outputs(perturbed, &[start_response], &step_responses, delta)
    }

    /// Borrow the scalar elementary effects.
    #[must_use]
    pub const fn effects(&self) -> &[T; PARAMETERS] {
        &self.effects[0]
    }
}

impl fmt::Display for ElementaryEffectsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRangeParameter { step, index } => write!(
                formatter,
                "trajectory step {step} perturbed parameter {index}, which is out of range"
            ),
            Self::DuplicateParameter { parameter } => write!(
                formatter,
                "trajectory perturbed parameter {parameter} more than once"
            ),
            Self::NonPositiveStep => {
                formatter.write_str("trajectory step size must be strictly positive")
            }
        }
    }
}

impl core::error::Error for ElementaryEffectsError {}

/// Online Morris elementary-effect screening.
///
/// Feed one elementary-effect vector per trajectory. The report exposes the
/// mean `mu`, the mean absolute `mu_star`, and the standard deviation
/// `sigma` of each parameter's effects: `mu_star` ranks influence, while a
/// large `sigma` marks nonlinear or interaction-driven parameters.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MorrisScreening<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> {
    count: u64,
    sums: [[T; PARAMETERS]; OUTPUTS],
    absolute_sums: [[T; PARAMETERS]; OUTPUTS],
    sums_of_squares: [[T; PARAMETERS]; OUTPUTS],
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize> Default
    for MorrisScreening<T, PARAMETERS, OUTPUTS>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    MorrisScreening<T, PARAMETERS, OUTPUTS>
{
    /// Construct empty.
    pub fn new() -> Self {
        Self {
            count: 0,
            sums: [[T::ZERO; PARAMETERS]; OUTPUTS],
            absolute_sums: [[T::ZERO; PARAMETERS]; OUTPUTS],
            sums_of_squares: [[T::ZERO; PARAMETERS]; OUTPUTS],
        }
    }

    /// Add one trajectory's elementary effects for every output.
    pub fn update_outputs(&mut self, effects: &[[T; PARAMETERS]; OUTPUTS]) {
        self.count += 1;
        for (output_sums, (output_absolute, (output_squares, effects))) in self.sums.iter_mut().zip(
            self.absolute_sums
                .iter_mut()
                .zip(self.sums_of_squares.iter_mut().zip(effects)),
        ) {
            for (((sum, absolute), squares), &effect) in output_sums
                .iter_mut()
                .zip(output_absolute.iter_mut())
                .zip(output_squares.iter_mut())
                .zip(effects.iter())
            {
                *sum += effect;
                *absolute += effect.abs();
                *squares += effect * effect;
            }
        }
    }

    /// Produce Morris statistics.
    ///
    /// # Errors
    ///
    /// Requires two effects per parameter so `sigma` is defined.
    #[expect(
        clippy::cast_precision_loss,
        reason = "the generic numeric contract represents observation counts in T"
    )]
    pub fn report(self) -> Result<MorrisReport<T, PARAMETERS, OUTPUTS>, InsufficientSamples> {
        if self.count < 2 {
            return Err(InsufficientSamples::new(2, self.count));
        }
        let mut mu = [[T::ZERO; PARAMETERS]; OUTPUTS];
        let mut mu_star = [[T::ZERO; PARAMETERS]; OUTPUTS];
        let mut sigma = [[T::ZERO; PARAMETERS]; OUTPUTS];
        let count = T::from_f64(self.count as f64);
        for (((mu_row, mu_star_row), sigma_row), (sums, (absolute_sums, sums_of_squares))) in mu
            .iter_mut()
            .zip(mu_star.iter_mut())
            .zip(sigma.iter_mut())
            .zip(
                self.sums
                    .iter()
                    .zip(self.absolute_sums.iter().zip(self.sums_of_squares.iter())),
            )
        {
            for ((((mu, mu_star), sigma), &sum), (&absolute_sum, &sum_of_squares)) in mu_row
                .iter_mut()
                .zip(mu_star_row.iter_mut())
                .zip(sigma_row.iter_mut())
                .zip(sums.iter())
                .zip(absolute_sums.iter().zip(sums_of_squares.iter()))
            {
                *mu = sum / count;
                *mu_star = absolute_sum / count;
                let mean_square = sum_of_squares / count;
                *sigma = (mean_square - *mu * *mu).max_scalar(T::ZERO).sqrt();
            }
        }
        Ok(MorrisReport {
            effect_count: self.count,
            mu,
            mu_star,
            sigma,
        })
    }
}

/// Morris elementary-effect screening report.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MorrisReport<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> {
    effect_count: u64,
    mu: [[T; PARAMETERS]; OUTPUTS],
    mu_star: [[T; PARAMETERS]; OUTPUTS],
    sigma: [[T; PARAMETERS]; OUTPUTS],
}

impl<T, const PARAMETERS: usize, const OUTPUTS: usize> MorrisReport<T, PARAMETERS, OUTPUTS> {
    /// Elementary effects per parameter.
    #[must_use]
    pub const fn effect_count(&self) -> u64 {
        self.effect_count
    }

    /// Mean elementary effect per parameter.
    #[must_use]
    pub const fn mu_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        &self.mu
    }

    /// Mean absolute elementary effect per parameter.
    #[must_use]
    pub const fn mu_star_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        &self.mu_star
    }

    /// Elementary-effect standard deviation per parameter.
    #[must_use]
    pub const fn sigma_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        &self.sigma
    }
}

impl<T, const PARAMETERS: usize> MorrisReport<T, PARAMETERS, 1> {
    /// Mean elementary effect for the single output.
    #[must_use]
    pub const fn mu(&self) -> &[T; PARAMETERS] {
        &self.mu[0]
    }

    /// Mean absolute elementary effect for the single output.
    #[must_use]
    pub const fn mu_star(&self) -> &[T; PARAMETERS] {
        &self.mu_star[0]
    }

    /// Elementary-effect standard deviation for the single output.
    #[must_use]
    pub const fn sigma(&self) -> &[T; PARAMETERS] {
        &self.sigma[0]
    }
}

impl<T: RealField, const PARAMETERS: usize> MorrisScreening<T, PARAMETERS, 1> {
    /// Add one scalar-output trajectory's elementary effects.
    pub fn update(&mut self, effects: &[T; PARAMETERS]) {
        self.update_outputs(&[*effects]);
    }
}

/// Online Saltelli first- and total-order Sobol' index estimator.
///
/// The A/B/`A_i^B` scheme draws `N` rows in an independent matrix `A` and `N`
/// rows in an independent matrix `B`, plus, for each parameter `i`, the
/// matrix `A_i^B` that agrees with `A` except that column `i` comes from `B`.
/// Feeding one row triple at a time keeps the accumulation online and
/// allocation free. With `V` the sample variance of the `A` responses, the
/// first-order estimate is `S_i = sum(f(B)(f(A_i^B) - f(A))) / (N V)` and the
/// total-order estimate is `S_Ti = sum((f(A) - f(A_i^B))^2) / (2 N V)`.
/// Finite-sample estimates are clamped to the unit interval, matching the
/// squared-correlation screening.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SobolIndices<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> {
    count: u64,
    a_sum: [T; OUTPUTS],
    a_sum_squares: [T; OUTPUTS],
    first_cross: [[T; PARAMETERS]; OUTPUTS],
    total_squares: [[T; PARAMETERS]; OUTPUTS],
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize> Default
    for SobolIndices<T, PARAMETERS, OUTPUTS>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    SobolIndices<T, PARAMETERS, OUTPUTS>
{
    /// Construct empty.
    pub fn new() -> Self {
        Self {
            count: 0,
            a_sum: [T::ZERO; OUTPUTS],
            a_sum_squares: [T::ZERO; OUTPUTS],
            first_cross: [[T::ZERO; PARAMETERS]; OUTPUTS],
            total_squares: [[T::ZERO; PARAMETERS]; OUTPUTS],
        }
    }

    /// Add one A/B/`A_i^B` row triple for every output.
    ///
    /// `base` is `f(A)`, `independent` is `f(B)`, and `recombined[i]` is
    /// `f(A_i^B)` for parameter `i`. The outer array indexes outputs.
    pub fn update_outputs(
        &mut self,
        base: &[T; OUTPUTS],
        independent: &[T; OUTPUTS],
        recombined: &[[T; PARAMETERS]; OUTPUTS],
    ) {
        self.count += 1;
        for (
            ((a_sum, a_sum_squares), (first_cross, total_squares)),
            ((&base, &independent), recombined),
        ) in self
            .a_sum
            .iter_mut()
            .zip(self.a_sum_squares.iter_mut())
            .zip(
                self.first_cross
                    .iter_mut()
                    .zip(self.total_squares.iter_mut()),
            )
            .zip(base.iter().zip(independent.iter()).zip(recombined.iter()))
        {
            *a_sum += base;
            *a_sum_squares += base * base;
            for ((first, total), &value) in first_cross
                .iter_mut()
                .zip(total_squares.iter_mut())
                .zip(recombined.iter())
            {
                *first += independent * (value - base);
                let difference = base - value;
                *total += difference * difference;
            }
        }
    }

    /// Produce first- and total-order indices.
    ///
    /// # Errors
    ///
    /// Requires two rows so the `A` variance is defined.
    #[expect(
        clippy::cast_precision_loss,
        reason = "the generic numeric contract represents observation counts in T"
    )]
    pub fn report(self) -> Result<SobolReport<T, PARAMETERS, OUTPUTS>, InsufficientSamples> {
        if self.count < 2 {
            return Err(InsufficientSamples::new(2, self.count));
        }
        let count = T::from_f64(self.count as f64);
        let mut first_order = [[T::ZERO; PARAMETERS]; OUTPUTS];
        let mut total_order = [[T::ZERO; PARAMETERS]; OUTPUTS];
        for ((((first_row, total_row), (&a_sum, &a_sum_squares)), first_cross), total_squares) in
            first_order
                .iter_mut()
                .zip(total_order.iter_mut())
                .zip(self.a_sum.iter().zip(self.a_sum_squares.iter()))
                .zip(self.first_cross.iter())
                .zip(self.total_squares.iter())
        {
            let mean = a_sum / count;
            let variance = (a_sum_squares / count - mean * mean).max_scalar(T::ZERO);
            if variance > T::ZERO {
                let first_scale = variance * count;
                let total_scale = variance * count * T::from_f64(2.0);
                for (((first, total), &cross), &squares) in first_row
                    .iter_mut()
                    .zip(total_row.iter_mut())
                    .zip(first_cross.iter())
                    .zip(total_squares.iter())
                {
                    *first = (cross / first_scale).clamp(T::ZERO, T::ONE);
                    *total = (squares / total_scale).clamp(T::ZERO, T::ONE);
                }
            }
        }
        Ok(SobolReport {
            sample_count: self.count,
            first_order,
            total_order,
        })
    }
}

/// Saltelli Sobol' index report.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SobolReport<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> {
    sample_count: u64,
    first_order: [[T; PARAMETERS]; OUTPUTS],
    total_order: [[T; PARAMETERS]; OUTPUTS],
}

impl<T, const PARAMETERS: usize, const OUTPUTS: usize> SobolReport<T, PARAMETERS, OUTPUTS> {
    /// Rows per matrix.
    #[must_use]
    pub const fn sample_count(&self) -> u64 {
        self.sample_count
    }

    /// First-order indices `S_i`.
    #[must_use]
    pub const fn first_order_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        &self.first_order
    }

    /// Total-order indices `S_Ti`.
    #[must_use]
    pub const fn total_order_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        &self.total_order
    }
}

impl<T, const PARAMETERS: usize> SobolReport<T, PARAMETERS, 1> {
    /// First-order indices for the single output.
    #[must_use]
    pub const fn first_order(&self) -> &[T; PARAMETERS] {
        &self.first_order[0]
    }

    /// Total-order indices for the single output.
    #[must_use]
    pub const fn total_order(&self) -> &[T; PARAMETERS] {
        &self.total_order[0]
    }
}

impl<T: RealField, const PARAMETERS: usize> SobolIndices<T, PARAMETERS, 1> {
    /// Add a scalar-output A/B/`A_i^B` row triple.
    pub fn update(&mut self, base: T, independent: T, recombined: &[T; PARAMETERS]) {
        self.update_outputs(&[base], &[independent], &[*recombined]);
    }
}
