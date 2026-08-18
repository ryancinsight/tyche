//! Allocation-free experimental designs.

mod indexing;
mod latin_hypercube;
mod sobol;

pub(super) use indexing::{counter_coordinate, design_index, stratum_index};

pub use latin_hypercube::LatinHypercube;
pub use sobol::{
    DigitalShift, RuntimeSampleError, RuntimeSobol, Sobol, SobolDimensionError, SobolDimensions,
    SobolRange, SobolRangeError, SobolScramble, Unscrambled,
};
