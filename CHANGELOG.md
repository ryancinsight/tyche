# Changelog

## 0.5.0

### Breaking

- The candidate carries current main's Eunomia 0.11 nominal scalar boundary;
  published Tyche 0.4.0 used Eunomia 0.10. An old-provider F64 caller no longer
  satisfies the new RealField bound. Update Tyche and the consumer's Eunomia
  dependency together; recreate external wrapper values at that shared
  provider boundary. Primitive f32/f64 correlation calls retain their API.
- This release remains held: current floating Git Eunomia 0.12 does not
  satisfy the recorded 0.11 requirement, and known consumers still require
  Tyche 0.4. Existing owners must reconcile the latest coherent graph and
  consumer requirements before merge/publication. See [ADR 0006](docs/adr/0006-correlation-provider-release-boundary.md).

### Fixed

- [patch] Preserve squared Pearson correlation when the accumulated moments
  are finite but their products overflow or underflow. The private report
  calculation normalizes exceptional products in the selected scalar's native
  precision. Public APIs, accumulator layout, observation order, and external
  dependency identities are unchanged. See [the numerical contract](docs/correlation-range.md).

## Unreleased

### Changed

- [patch] Follow the Consus provider to 0.4.0. `consus-core` and `consus-zarr`
  move from 0.3.0 to 0.4.0 and the lock resolves them at the published
  0.4.0 revision, so the two edges and the rest of the Consus cohort agree
  on one identity. Consus 0.4.0 changes the public HDF5 async reader
  bounds; Tyche reads no HDF5 async surface, so no caller migrates.

- Close the shared workflow-pin integration with exact PR CI `32343473062`,
  Pages build `32343473408`, post-merge CI `32343825023`, Deploy mdBook
  `32343825746`, dynamic Pages `32343823918`, and live HTTP 200 evidence.

- Refresh the Pages caller to Atlas reusable workflow `20c9398`, retaining the
  `tyche-core` executable book gate and its existing toolchain/output inputs.

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
- Keep the Consus adapter, Moirai adapter, and facade private in Cargo
  metadata; only `tyche-core` is currently a publishable registry artifact.

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
