//! Random-access sampling contracts, streams, and designs.

mod contract;
mod latin_hypercube;
mod sequence;

pub use contract::{Design, SampleIndexError};
pub use latin_hypercube::LatinHypercube;
pub use sequence::{Seed, SplitMix64, StandardNormal};
