# Tyche ownership gap audit

## Phase 0 closure

Tyche owns one complete vertical study design without runtime, format, array,
allocator, or physics ownership.

## Consumer residuals

- CFDrs `cfd-optim/design/space/sampling/sampler.rs` duplicates LHS with nested
  vectors; CFD candidate mapping stays local and population variance must stay
  explicit.
- Helios `helios-imaging/src/noise.rs` duplicates SplitMix64/Box-Muller; photon
  physics stays local and seeded compatibility needs a golden decision.
- Kwavers duplicates moments and conformal ranks. One conformal path uses the
  reversed lower tail; one sensitivity path labels squared correlation as both
  first-order and total Sobol. Model independence remains a Kwavers/Coeus duty.

## Provider residuals

- Moirai floating map/reduce grouping varies with worker count, so Tyche fills
  indexed slots and summarizes serially.
- Consus Store has no durability contract, and filesystem path validation is
  weaker than its documentation. Tyche validates keys before delegation.

Hosted CI, public identity, Atlas gitlink, and migrated-consumer behavior are
not yet evidence.
