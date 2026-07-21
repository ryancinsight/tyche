# Changelog

## Unreleased

### Breaking

- Counter streams now require an explicit typed domain and algorithm policy;
  Latin-hypercube and standard-normal samplers require explicit
  `SplitMix64`. Seeded vectors change to remove cross-algorithm coordinate
  aliases. See ADR 0003.
- `Study::sample` now preserves its typed `SampleIndexError`, and the Moirai
  adapter reports a malformed public `Design` implementation as
  `DispatchError::DesignContract` instead of entering a contained panic path.
  See ADR 0002 for migration details.

### Fixed

- Separate Latin-hypercube stride, offset, jitter, and normal-transform
  counter domains; native `f32` normal sampling no longer narrows uniforms
  generated in `f64`.
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

- Nonzero stream versions, sealed ZST domains and algorithms, exact replay
  vectors, and a Criterion counter/design performance instrument.
- Four-crate dependency-inverted Tyche workspace.
- Random-access Latin hypercube and counter-addressed uniform/normal streams.
- Cow metadata, const-generic spaces, GAT model responses, ordered moments,
  explicit variance policies, honest correlation screening, and corrected
  conformal rank.
- Borrowed Moirai and Consus adapters, proofs, tests, and documentation.
- Exact-width Latin-hypercube counts and committed nextest time budgets.
