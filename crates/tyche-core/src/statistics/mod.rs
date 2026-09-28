//! Canonically ordered online statistics and sensitivity screening.

mod error;
mod moments;
mod sensitivity;
mod variance;

use eunomia::RealField;

pub use error::InsufficientSamples;
pub use moments::Moments;
pub use sensitivity::{
    CorrelationKind, CorrelationScreening, ElementaryEffects, ElementaryEffectsError, MorrisKind,
    MorrisReport, MorrisScreening, OnlineEstimator, Report, SensitivityKind, SensitivityReport,
    SobolIndices, SobolKind, SobolReport,
};
pub use variance::{PopulationVariance, SampleVariance, VariancePolicy};

/// Observation count expressed in a statistic's scalar field.
///
/// This is the single home of the observation-count narrowing lint: every
/// estimator routes its `u64` count through here instead of repeating the
/// cast and its justification.
#[expect(
    clippy::cast_precision_loss,
    reason = "the generic numeric contract represents observation counts in T"
)]
pub(crate) fn count_as<T: RealField>(count: u64) -> T {
    T::from_f64(count as f64)
}
