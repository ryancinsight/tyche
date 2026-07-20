//! Consus-backed artifact adaptation for Tyche.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod archive;
mod contract;
mod key;

pub use archive::ConsusArchive;
pub use contract::{ArtifactRead, ArtifactWrite};
pub use key::{ArtifactKey, ArtifactKeyError};
