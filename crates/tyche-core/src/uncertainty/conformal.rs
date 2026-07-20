//! Finite-sample split-conformal rank.

use super::{ConformalError, PredictionInterval};
use core::cmp::Ordering;
use eunomia::RealField;

/// Validated split-conformal miscoverage policy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConformalCalibrator<T> {
    miscoverage: T,
}

impl<T: RealField> ConformalCalibrator<T> {
    /// Construct with `0 < alpha < 1`.
    ///
    /// # Errors
    ///
    /// Rejects invalid alpha.
    pub fn new(miscoverage: T) -> Result<Self, ConformalError<T>> {
        if !miscoverage.is_finite() || miscoverage <= T::ZERO || miscoverage >= T::ONE {
            return Err(ConformalError::InvalidMiscoverage(miscoverage));
        }
        Ok(Self { miscoverage })
    }
    /// Miscoverage.
    #[must_use]
    pub const fn miscoverage(self) -> T {
        self.miscoverage
    }
    /// Sort caller scores and return `ceil((n+1)(1-alpha))`, capped at `n`.
    ///
    /// # Errors
    ///
    /// Rejects empty, negative, or non-finite scores.
    pub fn calibrate_in_place(self, scores: &mut [T]) -> Result<T, ConformalError<T>> {
        validate_scores(scores)?;
        scores.sort_unstable_by(|left, right| {
            if left < right {
                Ordering::Less
            } else if left > right {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        Ok(self.select_sorted(scores))
    }
    /// Return the corrected quantile from an already nondecreasing score slice.
    ///
    /// This borrowed form performs no allocation or mutation. The selected
    /// position is the first integer rank not less than
    /// `ceil((n+1)(1-alpha))`; therefore it equals the finite-sample corrected
    /// order statistic and is capped by the final score.
    ///
    /// # Errors
    ///
    /// Rejects empty, negative, non-finite, or decreasing scores.
    pub fn calibrate_sorted(self, scores: &[T]) -> Result<T, ConformalError<T>> {
        validate_scores(scores)?;
        for (index, pair) in scores.windows(2).enumerate() {
            if pair[1] < pair[0] {
                return Err(ConformalError::InvalidScore {
                    index: index + 1,
                    value: pair[1],
                });
            }
        }
        Ok(self.select_sorted(scores))
    }
    /// Form a symmetric interval.
    #[must_use]
    pub fn interval(self, prediction: T, radius: T) -> PredictionInterval<T> {
        PredictionInterval::new(prediction - radius, prediction + radius)
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "structural ranks are embedded into T before native-precision arithmetic"
    )]
    fn select_sorted(self, scores: &[T]) -> T {
        let corrected_rank =
            (T::from_f64((scores.len() + 1) as f64) * (T::ONE - self.miscoverage)).ceil();
        scores
            .iter()
            .enumerate()
            .find(|(index, _)| T::from_f64((index + 1) as f64) >= corrected_rank)
            .map_or_else(
                || {
                    scores
                        .last()
                        .copied()
                        .expect("invariant: score validation rejects empty slices")
                },
                |(_, &score)| score,
            )
    }
}

fn validate_scores<T: RealField>(scores: &[T]) -> Result<(), ConformalError<T>> {
    if scores.is_empty() {
        return Err(ConformalError::EmptyScores);
    }
    for (index, &score) in scores.iter().enumerate() {
        if !score.is_finite() || score < T::ZERO {
            return Err(ConformalError::InvalidScore {
                index,
                value: score,
            });
        }
    }
    Ok(())
}
