//! Hierarchical integration harness for the Tyche core uncertainty contracts.
//!
//! The leaf modules retain their original contract assertions untouched;
//! one Cargo target replaces the previous flat target-per-file topology
//! (9 binaries linking the provider per build). Nextest still isolates per
//! test, and the committed timeout/serial-group profile applies unchanged:
//! test-name filters match the trailing test name with or without the
//! harness module prefix.

mod bootstrap;
mod conformal_laws;
mod discrete_sampling;
mod layout_allocation;
mod sampling_laws;
mod sobol_design;
mod statistics_laws;
mod stream_vectors;
mod study_contract;
