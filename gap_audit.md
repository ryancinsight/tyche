# Tyche ownership gap audit

## TYCHE-008 publication boundary

The release backlog requires `tyche-core` to be the publishable registry
artifact while the Consus adapter, Moirai adapter, and facade remain private.
The prior manifests inherited the workspace-level `publish = true` for all
four packages, while the README described those private packages as merely
unreleased. The three integration manifests now set `publish = false`, and
the README, CHANGELOG, checklist, and backlog agree with the Cargo metadata.
The remaining TYCHE-008 work is external registry configuration and exact
release evidence; this slice does not claim publication. Formatting and
locked no-dependency metadata pass. The locked Nextest gate is blocked before
compilation by the Atlas overlay's unused local patches requiring a lockfile
rewrite, so this change carries no focused-test claim.

## TYCHE-004 multi-output sensitivity closure (2026-08-13)

The scalar Morris and Saltelli estimators were already real and provider-owned;
the remaining sensitivity gap was the missing output dimension. `tyche-core`
now parameterizes correlation, Morris, and Saltelli accumulation and reports by
`OUTPUTS` with a default single-output specialization preserving existing calls.
The `update_outputs` surfaces retain fixed-size arrays and no per-observation
allocation. A two-output test verifies independent correlation vectors, Morris
`mu`/`mu_star`/`sigma`, and Saltelli first/total-order indices against linear
oracles. Local evidence: fmt, warning-denied Clippy, `cargo nextest run -p
tyche-core` 48/48, and 17/17 doctests pass.

The only remaining TYCHE-004 scope is a genuine trainable-model seam for
ensemble bagging; the current score-only model remains excluded from the
provider migration.

The multi-output estimator delivery is closed at source
`dc96f5ecd6af643e34f2146b9f3dbb49ba85dbae`, merged as provider default
`4a6f8cd495c78beaaa6e4081705b33ed0da8be9e`. Hosted verification and
supply-chain run `31675476210` passed; the book build run `31675476477` passed.
The trainable-model seam remains a separate correctness item because the
current ensemble model retains no trained model state.

## Phase 0 closure

Tyche owns one complete vertical study design without runtime, format, array,
allocator, or physics ownership.

## Consumer integration

- Helios merge `a7321f6c6d33114effe2edc698b7227ddbda960a` removes its
  SplitMix64/Box-Muller duplicate while keeping photon physics and sinogram
  indexing local. The package-local suite passes 27/27 tests. Hosted workspace
  verification was blocked before compilation by its pre-existing missing
  Apollo checkout.
- CFDrs merge `e980bea92ed5440a540477931d1afd8488eeeea0` removes the nested-vector
  LHS and direct `rand` dependency while keeping CFD candidate mapping local.
  The package-local suite passes 128/128 tests; isolated semver reconstruction
  is blocked by Gaia's published relative Leto path.
- Kwavers merge `a8d797cc8bc4b3f032be3f12f586e5e1269837a1`
  removes local conformal ranks and moment accumulation, routes `f32` and
  `f64` calibration through Tyche, and names squared Pearson correlation as
  screening rather than Sobol. Local suites pass 764/764 Analysis and
  1,251/1,251 solver tests. Final head `be5bd37f1` passes the complete hosted
  migration, architecture, feature, stable/beta/nightly, Miri, CUDA, solver,
  PINN, coverage, benchmark, documentation, and security matrix.
- Kwavers PR 304 merges as `9ad18523d0a936f3d32c2921dc3ff6fce2e35de9`
  from source `cc382dbc2243678fef55101aa106e9f8d7ad7bbf`. It deletes its
  local fixed collocation LHS and pseudo-Sobol generators and consumes Tyche's
  const-generic random-access designs. Kwavers retains measure-correct
  rectangle, disk, ball, boundary, and interface mapping. Local evidence is
  46/46 grid and 21/21 PINN geometry/config tests; 23 hosted checks, including
  code coverage, have succeeded while four full benchmark pairs continue on
  the exact source head.

## Provider residuals

- Moirai merge `91c802e` closes the final scoped-dispatch lifetime race exposed
  by Tyche's 257-item, seven-item-chunk contract. Tyche pins that revision and
  its exact former Windows access-violation case now passes.
- Moirai floating map/reduce grouping varies with worker count, so Tyche fills
  indexed slots and summarizes serially.
- Consus Store has no durability contract, and filesystem path validation is
  weaker than its documentation. Tyche validates keys before delegation.

Tyche baseline main `94d3c342b48045bda2364b1bc8d1d62d5e2ca99e` is hosted-CI green for both
verification and supply-chain policy. Its prior full local locked gate passes
the no-std check, warning-denied Clippy, 18/18 Nextest, 9/9 doctests, Rustdoc,
the reproducible-study example, and supply-chain policy.
`cargo-semver-checks` completes but has no published Tyche baseline to compare.
The remaining hosted evidence limits are explicit.

## Sampling breadth

- ADR 0003 closes the untyped counter namespace that aliased LHS coefficient,
  jitter, and normal coordinates. Public stream/design/distribution types now
  require an explicit algorithm ZST and exact versioned vectors.
- A controlled `origin/main` Criterion comparison detects no raw-word or
  normal-throughput regression and measures a 7.87% lower median for width-8
  LHS sampling. Repeated sampling remains allocation-free.
- ADR 0004 adds fixed and runtime Sobol over one const-generic kernel for the
  verified one-through-three-dimensional consumer boundary. Explicit ranges
  replace seed-derived skipping; `Unscrambled` and versioned `DigitalShift`
  are static policies. Exact vectors, 1,024-point sequential differential
  equivalence, projection stratification before and after shifting, fixed and
  runtime equality, failure atomicity, and row-major equivalence pass.
- Hoisting Gray-bit discovery from the dimension loop reduces Criterion's
  width-3 fixed estimate by 14.03%, runtime estimate by 27.78%, and 4,096-row
  fill estimate by 54.43%. The allocation gate covers every Sobol path and
  remains at zero post-construction allocations.
- `cargo-semver-checks` reports five major API changes against `origin/main`,
  matching ADR 0003's classification; a major-release check passes under a
  temporary metadata-only version projection. The delivered manifests remain
  at 0.1.0 because no release or version bump is authorized in this increment.
- ADR 0005 adds exact uniform categorical reduction, borrowed-or-owned
  validated mass tables, native-precision weighted inverse-CDF sampling, and
  support-checked discrete importance ratios. One generic contract suite
  covers `f32` and `f64`; exact replay, six-standard-error distribution laws,
  algebraic importance equality, invalid mass/support cases, `Cow` storage
  identity, and allocation/layout invariants are executable gates.
- The current branch passes warning-denied all-target/all-feature Clippy,
  40/40 workspace Nextest cases, 18/18 doctests, warning-denied Rustdoc, the
  end-to-end example, and supply-chain policy. On this Windows x86-64 machine,
  Criterion measures width-16 categorical, weighted, and importance medians of
  2.389 ns, 14.594 ns, and 15.872 ns respectively; these are raw instrument
  readings without an earlier discrete baseline, not speedup claims. Existing
  row-major Sobol throughput has no statistically significant change.
- `cargo-semver-checks` classifies the public discrete surface as additive:
  all 196 applicable minor-release checks pass against `origin/main`.
- Moirai and Consus adapters require no change until runtime-dimensional
  studies or versioned persistence enter their respective scopes.

## UQ breadth

- Tyche now owns deterministic bootstrap index generation: validated runtime
  population/resample sizes, random-access `Bootstrap::at`, caller-owned
  `fill_into`, a dedicated stream domain, and the shared exact multiply-high
  reducer. The provider does not own percentile interpolation or consumer
  confidence-interval policy.
- Kwavers' elastography percentile confidence interval now delegates
  deterministic index generation to Tyche's `Bootstrap::<SplitMix64>` through
  `crates/kwavers-analysis/src/signal_processing/estimation_bounds.rs`;
  percentile interpolation stays in Kwavers. The consumer source slice is
  format-clean and locked workspace metadata resolves. Its nominal
  entropy-seeded ensemble bagging remains a separate correctness item, not a
  bootstrap-API acceptance shortcut: `EnsembleModel::train` retains no trained
  model and prediction only perturbs an external predictor.
- The remaining consumer bootstrap work is therefore limited to any other
  consumer-specific confidence policies; Tyche's provider index-generation
  contract is consumed at this audited Kwavers boundary. Ensemble entropy and
  `rand` ownership remain separate until a real trainable-model seam exists.
