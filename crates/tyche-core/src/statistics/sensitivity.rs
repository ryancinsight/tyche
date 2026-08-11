//! Global sensitivity screening: squared correlation, Morris elementary
//! effects, and Saltelli first- and total-order Sobol' indices.

use core::fmt;

use super::InsufficientSamples;
use eunomia::RealField;

/// Online parameter-response correlation screening.
#[must_use]
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
#[must_use]
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

/// Morris trajectory elementary-effect batch.
///
/// A Morris trajectory walks `PARAMETERS + 1` design points: a shared start
/// point followed by one step per parameter. Each step changes exactly one
/// coordinate by `+delta` or `-delta`, and the response difference divided by
/// the step size is that parameter's elementary effect. This type validates
/// the trajectory contract and reduces one trajectory to its effect vector.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementaryEffects<T, const PARAMETERS: usize> {
    effects: [T; PARAMETERS],
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

impl<T: RealField, const PARAMETERS: usize> ElementaryEffects<T, PARAMETERS> {
    /// Reduce a validated trajectory to per-parameter elementary effects.
    ///
    /// `perturbed[step]` is the parameter index changed at each of the
    /// `PARAMETERS` steps and must name every parameter exactly once;
    /// `start_response` is the model response at the trajectory start, and
    /// `step_responses[step]` is the response immediately after the step that
    /// perturbed `perturbed[step]`. `delta` is the perturbation magnitude
    /// used for every step.
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
    pub fn from_steps(
        perturbed: &[usize; PARAMETERS],
        start_response: T,
        step_responses: &[T; PARAMETERS],
        delta: T,
    ) -> Result<Self, ElementaryEffectsError> {
        if !delta.is_finite() || delta <= T::ZERO {
            return Err(ElementaryEffectsError::NonPositiveStep);
        }
        let mut seen = [false; PARAMETERS];
        let mut effects = [T::ZERO; PARAMETERS];
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
            let previous = if step == 0 {
                start_response
            } else {
                step_responses[step - 1]
            };
            effects[parameter] = (step_responses[step] - previous) / delta;
        }
        Ok(Self { effects })
    }

    /// Borrow the per-parameter elementary effects.
    #[must_use]
    pub const fn effects(&self) -> &[T; PARAMETERS] {
        &self.effects
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
pub struct MorrisScreening<T, const PARAMETERS: usize> {
    count: u64,
    sums: [T; PARAMETERS],
    absolute_sums: [T; PARAMETERS],
    sums_of_squares: [T; PARAMETERS],
}

impl<T: RealField, const PARAMETERS: usize> Default for MorrisScreening<T, PARAMETERS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RealField, const PARAMETERS: usize> MorrisScreening<T, PARAMETERS> {
    /// Construct empty.
    pub fn new() -> Self {
        Self {
            count: 0,
            sums: [T::ZERO; PARAMETERS],
            absolute_sums: [T::ZERO; PARAMETERS],
            sums_of_squares: [T::ZERO; PARAMETERS],
        }
    }

    /// Add one trajectory's elementary effects.
    pub fn update(&mut self, effects: &[T; PARAMETERS]) {
        self.count += 1;
        for (dimension, &effect) in effects.iter().enumerate() {
            self.sums[dimension] += effect;
            self.absolute_sums[dimension] += effect.abs();
            self.sums_of_squares[dimension] += effect * effect;
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
    pub fn report(self) -> Result<MorrisReport<T, PARAMETERS>, InsufficientSamples> {
        if self.count < 2 {
            return Err(InsufficientSamples::new(2, self.count));
        }
        let mut mu = [T::ZERO; PARAMETERS];
        let mut mu_star = [T::ZERO; PARAMETERS];
        let mut sigma = [T::ZERO; PARAMETERS];
        let count = T::from_f64(self.count as f64);
        for dimension in 0..PARAMETERS {
            mu[dimension] = self.sums[dimension] / count;
            mu_star[dimension] = self.absolute_sums[dimension] / count;
            let mean_square = self.sums_of_squares[dimension] / count;
            sigma[dimension] = (mean_square - mu[dimension] * mu[dimension])
                .max_scalar(T::ZERO)
                .sqrt();
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
pub struct MorrisReport<T, const PARAMETERS: usize> {
    effect_count: u64,
    mu: [T; PARAMETERS],
    mu_star: [T; PARAMETERS],
    sigma: [T; PARAMETERS],
}

impl<T, const PARAMETERS: usize> MorrisReport<T, PARAMETERS> {
    /// Elementary effects per parameter.
    #[must_use]
    pub const fn effect_count(&self) -> u64 {
        self.effect_count
    }

    /// Mean elementary effect per parameter.
    #[must_use]
    pub const fn mu(&self) -> &[T; PARAMETERS] {
        &self.mu
    }

    /// Mean absolute elementary effect per parameter.
    #[must_use]
    pub const fn mu_star(&self) -> &[T; PARAMETERS] {
        &self.mu_star
    }

    /// Elementary-effect standard deviation per parameter.
    #[must_use]
    pub const fn sigma(&self) -> &[T; PARAMETERS] {
        &self.sigma
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
pub struct SobolIndices<T, const PARAMETERS: usize> {
    count: u64,
    a_sum: T,
    a_sum_squares: T,
    first_cross: [T; PARAMETERS],
    total_squares: [T; PARAMETERS],
}

impl<T: RealField, const PARAMETERS: usize> Default for SobolIndices<T, PARAMETERS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RealField, const PARAMETERS: usize> SobolIndices<T, PARAMETERS> {
    /// Construct empty.
    pub fn new() -> Self {
        Self {
            count: 0,
            a_sum: T::ZERO,
            a_sum_squares: T::ZERO,
            first_cross: [T::ZERO; PARAMETERS],
            total_squares: [T::ZERO; PARAMETERS],
        }
    }

    /// Add one A/B/`A_i^B` row triple.
    ///
    /// `base` is `f(A)`, `independent` is `f(B)`, and `recombined[i]` is
    /// `f(A_i^B)` for parameter `i`.
    pub fn update(&mut self, base: T, independent: T, recombined: &[T; PARAMETERS]) {
        self.count += 1;
        self.a_sum += base;
        self.a_sum_squares += base * base;
        for (dimension, &value) in recombined.iter().enumerate() {
            self.first_cross[dimension] += independent * (value - base);
            let difference = base - value;
            self.total_squares[dimension] += difference * difference;
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
    pub fn report(self) -> Result<SobolReport<T, PARAMETERS>, InsufficientSamples> {
        if self.count < 2 {
            return Err(InsufficientSamples::new(2, self.count));
        }
        let count = T::from_f64(self.count as f64);
        let mean = self.a_sum / count;
        let variance = (self.a_sum_squares / count - mean * mean).max_scalar(T::ZERO);
        let mut first_order = [T::ZERO; PARAMETERS];
        let mut total_order = [T::ZERO; PARAMETERS];
        if variance > T::ZERO {
            let first_scale = variance * count;
            let total_scale = variance * count * T::from_f64(2.0);
            for dimension in 0..PARAMETERS {
                first_order[dimension] =
                    (self.first_cross[dimension] / first_scale).clamp(T::ZERO, T::ONE);
                total_order[dimension] =
                    (self.total_squares[dimension] / total_scale).clamp(T::ZERO, T::ONE);
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
pub struct SobolReport<T, const PARAMETERS: usize> {
    sample_count: u64,
    first_order: [T; PARAMETERS],
    total_order: [T; PARAMETERS],
}

impl<T, const PARAMETERS: usize> SobolReport<T, PARAMETERS> {
    /// Rows per matrix.
    #[must_use]
    pub const fn sample_count(&self) -> u64 {
        self.sample_count
    }

    /// First-order indices `S_i`.
    #[must_use]
    pub const fn first_order(&self) -> &[T; PARAMETERS] {
        &self.first_order
    }

    /// Total-order indices `S_Ti`.
    #[must_use]
    pub const fn total_order(&self) -> &[T; PARAMETERS] {
        &self.total_order
    }
}
