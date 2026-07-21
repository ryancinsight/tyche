//! Random-access sampling contracts, typed streams, and designs.

mod contract;
mod counter;
mod distribution;
mod latin_hypercube;

pub use contract::{Design, SampleIndexError};
pub use counter::{
    Counter, SampleScalar, Seed, SplitMix64, StreamAlgorithm, StreamDomain, StreamVersion,
    UserDomain,
};
pub use distribution::StandardNormal;
pub use latin_hypercube::LatinHypercube;
