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
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        reason = "validated finite rank lies in the closed interval [1, scores.len()]"
    )]
    pub fn calibrate_in_place(self, scores: &mut [T]) -> Result<T, ConformalError<T>> {
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
        scores.sort_unstable_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
        let count = scores.len();
        let rank = (count + 1) as f64 * (1.0 - self.miscoverage.to_f64());
        let raw = <f64 as eunomia::FloatElement>::ceil(rank) as usize;
        Ok(scores[raw.clamp(1, count) - 1])
    }
    /// Form a symmetric interval.
    #[must_use]
    pub fn interval(self, prediction: T, radius: T) -> PredictionInterval<T> {
        PredictionInterval::new(prediction - radius, prediction + radius)
    }
}
