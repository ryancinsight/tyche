//! Saltelli Sobol sensitivity-index screening.

use eunomia::RealField;

use crate::statistics::InsufficientSamples;

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
