//! Online global sensitivity estimators.

mod correlation;
mod elementary;
mod estimator;
mod report;
mod sobol;

pub use correlation::CorrelationScreening;
pub use elementary::{ElementaryEffects, ElementaryEffectsError, MorrisScreening};
pub use estimator::OnlineEstimator;
pub use report::{
    CorrelationKind, MorrisKind, MorrisReport, Report, SensitivityKind, SensitivityReport,
    SobolKind, SobolReport,
};
pub use sobol::SobolIndices;

mod private {
    /// Seals the sensitivity estimator scaffold to Tyche's own estimators.
    pub trait Sealed {}
}
