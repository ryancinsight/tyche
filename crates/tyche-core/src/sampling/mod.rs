//! Random-access sampling contracts, typed streams, and designs.

mod contract;
mod counter;
mod design;
mod distribution;

pub use contract::{Design, SampleIndexError};
pub use counter::{
    Counter, SampleScalar, Seed, SplitMix64, StreamAlgorithm, StreamDomain, StreamVersion,
    UserDomain,
};
pub use design::{
    DigitalShift, LatinHypercube, RuntimeSampleError, RuntimeSobol, Sobol, SobolDimensionError,
    SobolDimensions, SobolRange, SobolRangeError, SobolScramble, Unscrambled,
};
pub use distribution::StandardNormal;
pub use distribution::{
    Categorical, CategoryCount, CategoryIndex, DiscreteImportance, DiscreteWeights,
    ImportanceError, ImportanceSample, WeightError, WeightedCategorical,
};
