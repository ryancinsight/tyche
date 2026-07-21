# Tyche ownership gap audit

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
  LHS sampling. Repeated sampling remains allocation-free. The current branch
  passes warning-denied all-target Clippy and 24/24 workspace Nextest cases.
- `cargo-semver-checks` reports five major API changes against `origin/main`,
  matching ADR 0003's classification; a major-release check passes. No release
  or version bump is authorized in this increment.
- Runtime-dimensional views, random-access Sobol, categorical, weighted, and
  discrete importance sampling remain TYCHE-003 work. Moirai and Consus
  adapters require no change until runtime-dimensional studies or versioned
  persistence enter their respective scopes.
