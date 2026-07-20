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
- Kwavers PR 298 removes local conformal ranks and moment accumulation, routes
  f32 and f64 calibration through Tyche, and names squared Pearson correlation
  as screening rather than Sobol. Local suites pass 718/718 analysis and
  1,251/1,251 solver tests. Hosted verification and merge remain.

## Provider residuals

- Moirai merge `91c802e` closes the final scoped-dispatch lifetime race exposed
  by Tyche's 257-item, seven-item-chunk contract. Tyche pins that revision and
  its exact former Windows access-violation case now passes.
- Moirai floating map/reduce grouping varies with worker count, so Tyche fills
  indexed slots and summarizes serially.
- Consus Store has no durability contract, and filesystem path validation is
  weaker than its documentation. Tyche validates keys before delegation.

Tyche main `c6b69addabf89cec73aa9feab2010e06b0c0a4a6` is hosted-CI green. Main
`90e466388a936e0f9fff56e33baae69b467144ed` failed only its formatting gate;
this delivery formats that scalar-generic expression and passes the full local
locked gate: no-std check, warning-denied Clippy, 17/17 Nextest, 9/9 doctests,
Rustdoc, the reproducible-study example, supply-chain policy, and SemVer checks.
The remaining hosted evidence limits are explicit.
