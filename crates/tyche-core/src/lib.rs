//! Atlas reproducible uncertainty-study foundation.
//!
//! Tyche owns study identity, parameter spaces, random-access experimental
//! designs, ensemble summaries, sensitivity screening, and conformal
//! calibration. Moirai owns execution and Consus owns persistence.

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate alloc;

/// Validated parameter and parameter-space contracts.
pub mod design;
/// Borrowed ensemble views and statically dispatched model contracts.
pub mod ensemble;
/// Deterministic random-access experimental designs.
pub mod sampling;
/// Online ensemble statistics and sensitivity screening.
pub mod statistics;
/// Reproducible study specifications and generated samples.
pub mod study;
/// Distribution-free uncertainty calibration.
pub mod uncertainty;

pub use design::{InvalidParameter, Parameter, ParameterSpace, SpaceError};
pub use ensemble::{Ensemble, ResponseReducer, StudyModel};
pub use sampling::{
    Counter, Design, DigitalShift, LatinHypercube, RuntimeSampleError, RuntimeSobol,
    SampleIndexError, SampleScalar, Seed, Sobol, SobolDimensionError, SobolDimensions, SobolRange,
    SobolRangeError, SobolScramble, SplitMix64, StandardNormal, StreamAlgorithm, StreamDomain,
    StreamVersion, Unscrambled, UserDomain,
};
pub use statistics::{
    CorrelationScreening, InsufficientSamples, Moments, PopulationVariance, SampleVariance,
    SensitivityReport, VariancePolicy,
};
pub use study::{Sample, Study, StudyError};
pub use uncertainty::{ConformalCalibrator, ConformalError, PredictionInterval};
