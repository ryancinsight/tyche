//! Saltelli Sobol sensitivity-index screening.

use eunomia::RealField;

use super::estimator::OnlineEstimator;
use super::private::Sealed;
use super::report::SobolReport;
use crate::statistics::{InsufficientSamples, Moments};

/// Online Saltelli first- and total-order Sobol' index estimator.
///
/// The A/B/`A_i^B` scheme draws `N` rows in an independent matrix `A` and `N`
/// rows in an independent matrix `B`, plus, for each parameter `i`, the
/// matrix `A_i^B` that agrees with `A` except that column `i` comes from `B`.
/// Feeding one row triple at a time keeps the accumulation online and
/// allocation free. With `V` the population variance of the `A` responses, the
/// first-order estimate is `S_i = sum(f(B)(f(A_i^B) - f(A))) / (N V)` and the
/// total-order estimate is `S_Ti = sum((f(A) - f(A_i^B))^2) / (2 N V)`.
/// Finite-sample estimates are clamped to the unit interval, matching the
/// squared-correlation screening.
///
/// The denominator uses the canonical centered online moments, avoiding the
/// cancellation in `E[A²] - E[A]²` for responses with a large constant offset.
/// This does not make the finite-sample first-order numerator translation
/// invariant or extend the scalar range of its accumulated products.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SobolIndices<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> {
    count: u64,
    a_moments: [Moments<T>; OUTPUTS],
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

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize> Sealed
    for SobolIndices<T, PARAMETERS, OUTPUTS>
{
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    SobolIndices<T, PARAMETERS, OUTPUTS>
{
    /// Construct empty.
    pub fn new() -> Self {
        Self {
            count: 0,
            a_moments: [Moments::new(); OUTPUTS],
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
        OnlineEstimator::accumulate(self, (base, independent, recombined));
    }

    /// Produce first- and total-order indices.
    ///
    /// # Errors
    ///
    /// Requires two rows so the `A` variance is defined.
    pub fn report(self) -> Result<SobolReport<T, PARAMETERS, OUTPUTS>, InsufficientSamples> {
        OnlineEstimator::report(self)
    }
}

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    OnlineEstimator<T, PARAMETERS, OUTPUTS> for SobolIndices<T, PARAMETERS, OUTPUTS>
{
    type Report = SobolReport<T, PARAMETERS, OUTPUTS>;
    type Observation<'a> = (
        &'a [T; OUTPUTS],
        &'a [T; OUTPUTS],
        &'a [[T; PARAMETERS]; OUTPUTS],
    );
    const MINIMUM_SAMPLES: u64 = 2;

    fn sample_count(&self) -> u64 {
        self.count
    }

    fn accumulate(&mut self, (base, independent, recombined): Self::Observation<'_>) {
        self.count += 1;
        for ((a_moments, (first_cross, total_squares)), ((&base, &independent), recombined)) in self
            .a_moments
            .iter_mut()
            .zip(
                self.first_cross
                    .iter_mut()
                    .zip(self.total_squares.iter_mut()),
            )
            .zip(base.iter().zip(independent.iter()).zip(recombined.iter()))
        {
            a_moments.update(base);
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

    fn finish(self) -> Self::Report {
        let mut first_order = [[T::ZERO; PARAMETERS]; OUTPUTS];
        let mut total_order = [[T::ZERO; PARAMETERS]; OUTPUTS];
        for ((((first_row, total_row), a_moments), first_cross), total_squares) in first_order
            .iter_mut()
            .zip(total_order.iter_mut())
            .zip(self.a_moments.iter())
            .zip(self.first_cross.iter())
            .zip(self.total_squares.iter())
        {
            // M2 = N * V for the population variance. Use it directly:
            // dividing by N and multiplying back adds avoidable rounding.
            let centered_sum = a_moments.centered_sum();
            if centered_sum > T::ZERO {
                for (((first, total), &cross), &squares) in first_row
                    .iter_mut()
                    .zip(total_row.iter_mut())
                    .zip(first_cross.iter())
                    .zip(total_squares.iter())
                {
                    *first = (cross / centered_sum).clamp(T::ZERO, T::ONE);
                    // Divide by two last so a finite M2 is not overflowed
                    // solely by forming the denominator 2 * M2.
                    *total = ((squares / centered_sum) / T::from_f64(2.0)).clamp(T::ZERO, T::ONE);
                }
            }
        }
        SobolReport::from_indices(self.count, first_order, total_order)
    }
}

impl<T: RealField, const PARAMETERS: usize> SobolIndices<T, PARAMETERS, 1> {
    /// Add a scalar-output A/B/`A_i^B` row triple.
    pub fn update(&mut self, base: T, independent: T, recombined: &[T; PARAMETERS]) {
        self.update_outputs(&[base], &[independent], &[*recombined]);
    }
}
