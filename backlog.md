# Tyche implementation backlog

## ATLAS-TYCHE-WORKFLOW-PIN-2026-08-20 — Refresh shared book workflow pin [patch] — done 2026-08-20

- Owner: Atlas integration. Scope is the Pages caller and this PM/changelog
  record; Tyche implementation, release work, and lockfile remain unchanged.
- Acceptance: pin the caller to the current Atlas reusable workflow while
  retaining `mdbook-test: true`, Rust 1.97.0, package `tyche-core`, and the
  existing output path, then pass exact hosted source and Pages checks before
  advancing the Atlas gitlink.
- Evidence: source `f98ecb14bc5527e3a774a5d4b2bbd109cf5d9157`, PR #28, merged
  default `46f4829ef648cec2b9e44bad3a75aef8ef3c34af`; exact CI run
  `32343473062` and Pages build `32343473408` pass. Post-merge CI
  `32343825023`, Deploy mdBook `32343825746`, and dynamic Pages
  `32343823918` pass; live Pages returns HTTP 200 with title `Tyche | tyche`.
- Delivery: the Pages caller is pinned to Atlas workflow
  `1fcd17c6f7923cb1734756c15e0a5a39e333ee32`; the Atlas gitlink may advance to
  the merged default after this PM closure merges.

## TYCHE-008 — crates.io release automation — in progress

- Owner: Codex `/root`; scope: publish `tyche-core` from an exact merged source
  revision, configure crates.io Trusted Publishing for the repository release
  workflow, and keep the adapter and facade crates private.
- Acceptance: standalone locked package validation and focused tests pass; the
  crate is indexed; the exact-source GitHub Release exists; crates.io accepts
  only trusted-publisher updates.

## TYCHE-007 — Provider source consolidation — implemented

- Resolve Eunomia 0.7 through its canonical versioned Git source; no sampling,
  statistics, or public API behavior changes.
- Locked metadata contains one Eunomia package, and workspace format, check,
  warning-denied Clippy, Nextest, doctest, and Rustdoc gates pass.

## TYCHE-001 — Phase 0 core — implemented

- Reproducible design, study, ensemble, statistics, calibration, execution, and
  artifact-access vertical slice.

## TYCHE-002 — Public promotion — implemented

- Public `ryancinsight/tyche` origin and the Atlas gitlink are registered.

## TYCHE-006 — Consumer migrations — complete

- Owner: Codex; PM closeout delivered on `codex/tyche-pm-closeout`; scope was
  to document merged Kwavers PR 304's direct Tyche collocation-design adoption
  in `README.md`, `backlog.md`, `gap_audit.md`, and `CHANGELOG.md`. Provider
  code, unrelated consumer surfaces, and release metadata remain non-goals.

- Helios PR 10 delegates reproducible normal noise to `StandardNormal`; its
  package-local suite passes 27/27 tests.
- CFDrs PR 299 delegates const-width Latin-hypercube designs to Tyche; its
  package-local suite passes 128/128 tests.
- Kwavers merge `a8d797cc8bc4b3f032be3f12f586e5e1269837a1`
  delegates conformal calibration, moments, and correlation screening to
  Tyche. The local all-feature Analysis suite passes 764/764 tests, and the
  solver suite passes 1,251/1,251 tests. Final head `be5bd37f1` passes the
  complete hosted migration, architecture, feature, stable/beta/nightly,
  Miri, CUDA, solver, PINN, coverage, benchmark, documentation, and
  security matrix.
- Kwavers PR 304 merges as `9ad18523d0a936f3d32c2921dc3ff6fce2e35de9`
  from source `cc382dbc2243678fef55101aa106e9f8d7ad7bbf`. It delegates
  fixed Latin-hypercube and Sobol collocation designs to Tyche while retaining
  rectangle, disk, ball, interface, and physics mappings in Kwavers. Local
  value-semantic suites pass 46/46 grid tests and 21/21 PINN geometry/config
  tests; the exact source head has 23 successful hosted checks, including code
  coverage, while four full benchmark pairs remain in progress.
- Moirai merge `91c802e` repairs the final-completion lifetime race exposed by
  Tyche's 257-item, seven-item-chunk adapter contract. The pinned Tyche
  workspace passes 18/18 tests, including the exact former access-violation
  case.
- ADR 0002 closes the public `Design` error-construction gap and replaces the
  adapter's contained panic path with a typed `DesignContract` failure.
- **Status:** complete. The required consumer evidence is synchronized in the
  four named records, and the current Tyche default `92cb29f` passes CI run
  `31645405288` (format, check, warning-denied Clippy, Nextest, doctests,
  Rustdoc, example, and supply-chain checks). The four Kwavers benchmark pairs
  remain a consumer-performance watchpoint outside this documentation item.

## TYCHE-003 — Sampling breadth — implemented

- Owner: `/root`; scope: `tyche-core` sampling/design modules, their tests,
  performance evidence, ADRs, README, changelog, and facade exports.

- Domain-separated, explicitly versioned stream vectors are implemented with
  native-precision unit conversion and controlled performance evidence.
- Fixed and runtime random-access Sobol designs share one const-generic kernel,
  explicit sequence ranges, static scrambling policies, typed failures, and an
  allocation-free row-major fill. Exact vectors, sequential differential
  checks, dyadic projection laws, and controlled performance evidence pass.
- ADR 0005 delivers categorical, weighted, and discrete importance sampling
  with typed validation, exact categorical reduction, `Cow` mass storage,
  native-precision arithmetic, and allocation-free repeated draws.
- Exact replay, generic `f32`/`f64` contracts, analytical importance identity,
  empirical laws with derived bounds, support failures, and allocation/layout
  invariants pass in the 40/40 workspace suite. All 18 doctests, warning-denied
  Clippy/Rustdoc, the end-to-end example, supply-chain policy, Criterion, and
  all 196 applicable additive SemVer checks pass.

## TYCHE-004 — UQ breadth — complete 2026-08-13

- Sensitivity-estimator source `dc96f5ecd6af643e34f2146b9f3dbb49ba85dbae`
  merged through PR #18 as provider default `4a6f8cd495c78beaaa6e4081705b33ed0da8be9e`.
  Provider verification and supply-chain checks passed in hosted run
  `31675476210`; book build passed in `31675476477`.

- First increment delivered in `tyche-core`: `Bootstrap` validates runtime
  population and resample sizes, exposes deterministic random-access indices,
  fills caller-owned storage without allocation, and reuses the canonical
  multiply-high bounded-integer kernel under a dedicated bootstrap domain.
  Provider-owned index generation is now complete; percentile interpolation
  remains a consumer policy.
- Consumer closure: Kwavers' elastography percentile bootstrap now delegates
  deterministic index generation to `Bootstrap::<SplitMix64>` in
  `tyche-core`; percentile interpolation remains Kwavers-owned. The consumer
  source slice is `crates/kwavers-analysis/src/signal_processing/
  estimation_bounds.rs`, with locked metadata and source formatting verified.
  This is a replay-boundary migration: consumers persisting results must record
  `SplitMix64::VERSION` and must not compare new output vectors with the former
  local continuous modulo schedule as if they were identical.
  The nominal ensemble bagging path remains separate and is not a valid
  migration target until it owns a real trainable-model seam: the current
  model stores only an error-derived score and prediction perturbs an external
  predictor without retained training.
- Delivered in the sensitivity-estimator increment: genuine Morris elementary
  effects and Saltelli first/total-order Sobol estimators, merged through PR #9
  as `deabe0b`; the current default CI run `31645405288` passes the full
  provider gate.
- Multi-output correlation, Morris, and Saltelli reports are delivered in the
  current increment through an `OUTPUTS` const-generic dimension. Scalar APIs
  remain the default specialization; two-output analytical laws pass in the
  `tyche-core` package suite.
- Remaining scope: a real trainable-model seam for ensemble bagging. The
  current score-only model is not represented as a trainable implementation.

## TYCHE-005 — Study schema — planned

- Versioned metadata/payload schema and manifest-last completeness; durability
  waits for Consus transaction support.
