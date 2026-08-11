//! Canonically ordered online statistics and sensitivity screening.

mod error;
mod moments;
mod sensitivity;
mod variance;

pub use error::InsufficientSamples;
pub use moments::Moments;
pub use sensitivity::{
    CorrelationScreening, ElementaryEffects, ElementaryEffectsError, MorrisReport, MorrisScreening,
    SensitivityReport, SobolIndices, SobolReport,
};
pub use variance::{PopulationVariance, SampleVariance, VariancePolicy};
