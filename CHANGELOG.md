# Changelog

## Unreleased

### Breaking

- `Study::sample` now preserves its typed `SampleIndexError`, and the Moirai
  adapter reports a malformed public `Design` implementation as
  `DispatchError::DesignContract` instead of entering a contained panic path.
  See ADR 0002 for migration details.

### Fixed

- Conformal ranks now compute in the caller's scalar precision, and sorted
  calibration scores support allocation-free borrowed quantile selection.
- Pin the Moirai adapter to revision `91c802e`, whose final scoped-dispatch
  completion handshake prevents a waiter from destroying scope state while the
  completing worker still holds a reference to it.

### Changed

- Helios, CFDrs, and Kwavers consumer boundaries now delegate reproducible
  normal sampling, Latin-hypercube designs, conformal calibration, moments,
  and correlation screening to Tyche without retaining local algorithm copies.

### Added

- Four-crate dependency-inverted Tyche workspace.
- Random-access Latin hypercube and counter-addressed uniform/normal streams.
- Cow metadata, const-generic spaces, GAT model responses, ordered moments,
  explicit variance policies, honest correlation screening, and corrected
  conformal rank.
- Borrowed Moirai and Consus adapters, proofs, tests, and documentation.
- Exact-width Latin-hypercube counts and committed nextest time budgets.
