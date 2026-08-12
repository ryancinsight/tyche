//! Curated Tyche facade for reproducible uncertainty studies.
//!
//! Re-exports `tyche-core`, which owns the backend-neutral study law:
//! validated parameter spaces, deterministic random-access Latin hypercube and
//! Sobol designs, fixed and runtime dimension selection, and deterministic
//! bootstrap resampling with caller-owned output.
//!
//! Two optional integrations sit behind features, both on by default:
//!
//! * `moirai` — [`MoiraiDispatch`] evaluates a study across the Moirai
//!   runtime; the dispatch contract keeps sample order deterministic
//!   regardless of completion order.
//! * `consus` — [`ConsusArchive`] and the [`ArtifactRead`]/[`ArtifactWrite`]
//!   pair persist a study and its outputs through Consus storage.
//!
//! Disable default features for a `no_std` core with neither integration.
//!
//! The repository README carries the prose introduction and worked examples;
//! it is linked from the crate's registry page rather than inlined here, so
//! that this documentation stays the API contract.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub use tyche_core::*;

#[cfg(feature = "consus")]
pub use tyche_consus::{ArtifactKey, ArtifactKeyError, ArtifactRead, ArtifactWrite, ConsusArchive};
#[cfg(feature = "moirai")]
pub use tyche_moirai::{DispatchError, MoiraiDispatch};
