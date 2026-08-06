//! Typed counter-addressed pseudorandom streams.

mod bounded;
mod domain;
mod scalar;
mod seed;
mod splitmix;

pub use domain::{StreamDomain, UserDomain};
pub use scalar::SampleScalar;
pub use seed::Seed;
pub use splitmix::{Counter, SplitMix64, StreamAlgorithm, StreamVersion};

pub(in crate::sampling) use bounded::bounded_u64;
pub(in crate::sampling) use domain::{
    BootstrapIndex, CategoricalSelection, LatinHypercubeJitter, LatinHypercubeOffset,
    LatinHypercubeStride, SobolDigitalShift, StandardNormalAngle, StandardNormalRadius,
    WeightedSelection,
};
