//! Typed counter-addressed pseudorandom streams.

mod domain;
mod scalar;
mod seed;
mod splitmix;

pub use domain::{StreamDomain, UserDomain};
pub use scalar::SampleScalar;
pub use seed::Seed;
pub use splitmix::{Counter, SplitMix64, StreamAlgorithm, StreamVersion};

pub(in crate::sampling) use domain::{
    LatinHypercubeJitter, LatinHypercubeOffset, LatinHypercubeStride, SobolDigitalShift,
    StandardNormalAngle, StandardNormalRadius,
};
