//! Allocation-free experimental designs.

mod latin_hypercube;
mod sobol;

pub use latin_hypercube::LatinHypercube;
pub use sobol::{
    DigitalShift, RuntimeSampleError, RuntimeSobol, Sobol, SobolDimensionError, SobolDimensions,
    SobolRange, SobolRangeError, SobolScramble, Unscrambled,
};
