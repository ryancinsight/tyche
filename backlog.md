# Tyche implementation backlog

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

## TYCHE-006 — Consumer migrations — implemented

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
- Moirai merge `91c802e` repairs the final-completion lifetime race exposed by
  Tyche's 257-item, seven-item-chunk adapter contract. The pinned Tyche
  workspace passes 18/18 tests, including the exact former access-violation
  case.
- ADR 0002 closes the public `Design` error-construction gap and replaces the
  adapter's contained panic path with a typed `DesignContract` failure.

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

## TYCHE-004 — UQ breadth — planned

- Reproducible bootstrap, Morris, true Saltelli Sobol, and multi-output reports.

## TYCHE-005 — Study schema — planned

- Versioned metadata/payload schema and manifest-last completeness; durability
  waits for Consus transaction support.
