//! Finite-sample split-conformal rank.

use core::cmp::Ordering;

use eunomia::{FloatElement, RealField};

use super::{ConformalError, PredictionInterval};

/// Validated split-conformal miscoverage policy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConformalCalibrator<T> {
    miscoverage: T,
}

impl<T: RealField> ConformalCalibrator<T> {
    /// Construct a calibrator with `0 < miscoverage < 1`.
    ///
    /// # Errors
    ///
    /// Returns [`ConformalError::InvalidMiscoverage`] outside that interval.
    pub fn new(miscoverage: T) -> Result<Self, ConformalError<T>> {
        if !miscoverage.is_finite() || miscoverage <= T::ZERO || miscoverage >= T::ONE {
            return Err(ConformalError::InvalidMiscoverage(miscoverage));
        }
        Ok(Self { miscoverage })
    }

    /// Miscoverage probability `alpha`.
    #[must_use]
    pub const fn miscoverage(self) -> T {
        self.miscoverage
    }

    /// Sort caller-owned scores in place and return the corrected quantile.
    ///
    /// The one-based rank is `ceil((n + 1)(1 - alpha))`, capped at `n`.
    /// No score copy or auxiliary allocation is performed.
    ///
    /// # Errors
    ///
    /// Returns [`ConformalError`] for an empty or invalid score set.
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
        let raw_rank = ((count + 1) as f64 * (1.0 - self.miscoverage.to_f64())).ceil() as usize;
        let one_based_rank = raw_rank.clamp(1, count);
        Ok(scores[one_based_rank - 1])
    }

    /// Form a symmetric interval around one prediction.
    #[must_use]
    pub fn interval(self, prediction: T, radius: T) -> PredictionInterval<T> {
        PredictionInterval::new(prediction - radius, prediction + radius)
    }
}
