//! Curated Tyche facade.

#![doc = include_str!("../../../README.md")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub use tyche_core::*;

#[cfg(feature = "consus")]
pub use tyche_consus::{ArtifactKey, ArtifactKeyError, ArtifactRead, ArtifactWrite, ConsusArchive};
#[cfg(feature = "moirai")]
pub use tyche_moirai::{DispatchError, MoiraiDispatch};
