//! Shared accumulation scaffold for the online sensitivity estimators.
//!
//! The three sensitivity estimators differ only in their accumulator
//! arithmetic and report shape. This trait owns everything else: the
//! observation count as a scalar, the minimum-sample guard, and the
//! count-to-report hand-off, so each estimator keeps just its verbatim
//! accumulation and finish arithmetic.

use eunomia::RealField;

use super::private::Sealed;
use crate::statistics::{InsufficientSamples, count_as};

/// An online estimator that reduces a stream of observations to one report.
///
/// Implementors own only their accumulator state. This trait owns the shared
/// contract around it: the minimum observation count, the count-to-scalar
/// conversion, and the undersampling guard that fronts [`Self::finish`].
pub trait OnlineEstimator<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>:
    Sealed + Copy
{
    /// Report produced from a fully accumulated estimator.
    type Report;

    /// One accumulation step, borrowing the caller's observation.
    type Observation<'a>;

    /// Minimum observations required before a report is defined.
    const MINIMUM_SAMPLES: u64;

    /// Observations accumulated so far.
    fn sample_count(&self) -> u64;

    /// Fold one observation into the accumulator.
    fn accumulate(&mut self, observation: Self::Observation<'_>);

    /// Build the report from an already-validated accumulator.
    fn finish(self) -> Self::Report;

    /// Observation count expressed in the estimator's scalar field.
    fn count_as(self) -> T {
        count_as(self.sample_count())
    }

    /// Build the report, rejecting an undersampled accumulator.
    ///
    /// # Errors
    ///
    /// Returns [`InsufficientSamples`] while fewer than
    /// [`Self::MINIMUM_SAMPLES`] observations have been folded in.
    fn report(self) -> Result<Self::Report, InsufficientSamples> {
        if self.sample_count() < Self::MINIMUM_SAMPLES {
            return Err(InsufficientSamples::new(
                Self::MINIMUM_SAMPLES,
                self.sample_count(),
            ));
        }
        Ok(self.finish())
    }
}
