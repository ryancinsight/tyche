# Changelog

## Unreleased

### Changed

- Refresh the Pages caller to the current Atlas reusable workflow revision;
  retain the existing package-staged executable book gate.

- [patch] Split the sensitivity estimators into dedicated correlation,
  elementary-effect, and Sobol modules; each module stays below the repository
  file-size target while the public re-export surface remains unchanged.

### Added

- Morris elementary-effect screening: `ElementaryEffects::from_steps` reduces
  a validated trajectory (one perturbation per parameter, strictly positive
  step) to per-parameter effects, and `MorrisScreening` accumulates them into
  a `MorrisReport` with `mu`, `mu_star`, and `sigma`. Two effects per
  parameter are required so `sigma` is defined.
- Saltelli first- and total-order Sobol' index estimation: `SobolIndices`
  folds one A/B/`A_i^B` row triple at a time and `SobolReport` exposes
  `S_i = sum(f(B)(f(A_i^B) - f(A))) / (N V)` and
  `S_Ti = sum((f(A) - f(A_i^B))^2) / (2 N V)` with unit-interval clamping.
  Independent A and B matrices are required; the integration tests draw them
  from distinct `UserDomain`-tagged `Counter`/`SplitMix64` streams.
- Multi-output correlation, Morris, and Saltelli reports: add an `OUTPUTS`
  const-generic dimension with allocation-free `update_outputs` methods while
  preserving the scalar APIs as the default specialization. Value-semantic
  two-output laws cover each output independently.
- Book chapters for moments, parameter spaces, Sobol sequences, sensitivity
  screening, and the Atlas stack position are now delivered prose.

### Distribution

- Publish `tyche-core` through a tag-gated crates.io Trusted Publishing
  workflow. Committed stack-local source overlays no longer affect standalone
  package resolution.

### Fixed

- [patch] Corrected stale `"0.5.0"` version requirements for `moirai-core` and
  `moirai-executor` in the workspace; the upstream crates are `0.4.0`, so
  `Cargo.toml` now declares `"0.4.0"` to match. Tests and benchmarks now
  discover their targets.

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

- Eunomia now resolves through its canonical versioned Git contract. The
  lockfile remains the reproducible revision pin without creating a distinct
  revision-qualified provider identity in consumers.
- Helios, CFDrs, and Kwavers consumer boundaries now delegate reproducible
  normal sampling, Latin-hypercube and fixed Sobol designs, conformal
  calibration, moments, and correlation screening to Tyche without retaining
  local algorithm copies. Kwavers retains its geometry and physics mappings.

### Added

- Deterministic `Bootstrap` resampling validates population and resample sizes,
  provides random-access indices, fills caller-owned output, and shares the
  exact multiply-high reducer with categorical sampling. The provider owns
  index generation; percentile interpolation remains a consumer policy.
- Uniform categorical sampling with exact multiply-high rejection, borrowed
  or owned validated discrete masses, native-precision weighted inverse-CDF
  sampling, and support-checked discrete importance ratios. Repeated draws are
  allocation-free. See ADR 0005.
- Fixed and runtime one-through-three-dimensional random-access Sobol designs,
  explicit validated sequence ranges, unscrambled and versioned digital-shift
  policies, typed dimension/output failures, and an allocation-free row-major
  fill over one const-generic kernel. See ADR 0004.
- Nonzero stream versions, sealed ZST domains and algorithms, exact replay
  vectors, and a Criterion counter/design performance instrument.
- Four-crate dependency-inverted Tyche workspace.
- Random-access Latin hypercube and counter-addressed uniform/normal streams.
- Cow metadata, const-generic spaces, GAT model responses, ordered moments,
  explicit variance policies, honest correlation screening, and corrected
  conformal rank.
- Borrowed Moirai and Consus adapters, proofs, tests, and documentation.
- Exact-width Latin-hypercube counts and committed nextest time budgets.
