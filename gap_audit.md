# Tyche ownership gap audit

## Phase 0 closure

Tyche now owns one complete vertical study design: named const-width parameter
spaces, deterministic random-access Latin hypercube points, borrowed model
evaluation, index-preserving Moirai dispatch, ordered statistics, conformal
calibration, and Consus artifact access. Core contains no array, runtime,
storage-format, allocator, or domain-physics ownership.

## Consumer residuals

### CFDrs

- `cfd-optim/design/space/sampling/sampler.rs` duplicates Latin hypercube
  mechanics and heap-allocates nested vectors. Its surrounding module is
  currently not activated by `design/space/mod.rs`.
- Robustness summaries use population variance and must migrate with
  `PopulationVariance`, not an implicit default.
- CFD-specific parameter-to-candidate mapping remains in CFDrs.

### Helios

- `helios-imaging/src/noise.rs` owns a private SplitMix64/Box-Muller stream.
- The photon-counting transform remains Helios physics; only the random stream
  migrates.
- Existing seeded outputs need a golden differential decision because the
  legacy stream advances before its first draw.

### Kwavers

- Analysis and solver trees duplicate moments, ensembles, conformal ranks, and
  sampling.
- One conformal implementation selects the reversed lower tail; the other uses
  the correct finite-sample rank.
- A sensitivity implementation labels squared correlation as first-order and
  total Sobol sensitivity, entropy-seeds bootstrap, and hard-codes ten Morris
  parameters.
- Model construction, dropout semantics, tensors, and domain response types
  stay in Kwavers/Coeus. Tyche aggregates caller-supplied independent results;
  it does not claim independence for cloned or repeated models.

## Provider residuals

- Moirai floating map/reduce grouping depends on worker count. Tyche therefore
  dispatches into index-addressed slots and reduces serially in index order.
- Consus `Store` has no transaction or durability capability. A future
  manifest-last schema can prove logical completeness, not crash durability.
- Consus filesystem stores do not centrally validate every documented path
  condition. `ArtifactKey` rejects traversal and platform paths before
  delegation.

## Evidence tiers

- Implemented theorem/contract evidence: core unit/property/layout/allocation
  tests and Moirai/Consus adapter tests.
- Integration evidence: package-scoped compilation and tests against pinned
  Moirai/Consus revisions.
- Not yet evidence: hosted CI, public remote identity, Atlas gitlink, or
  consumer behavior after migration.
