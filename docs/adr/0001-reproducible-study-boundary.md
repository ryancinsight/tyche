# ADR 0001: Reproducible uncertainty-study boundary

- Status: accepted
- Change class: minor, architectural
- Date: 2026-07-20

## Context

Atlas integrators repeat seeded sampling, design matrices, ensemble statistics,
conformal ranks, and sensitivity loops. The repetitions disagree about
population versus sample variance, sometimes derive randomness from mutable
execution order, and in one case label squared correlation as both first-order
and total Sobol sensitivity.

Moirai already owns execution and synchronization. Consus already owns
scientific storage and object-store access. Tyche must compose those providers,
not reimplement their mechanics or make them depend on uncertainty semantics.

## Decision

Use four crates with inward dependencies:

```text
tyche facade
├── tyche-core
├── tyche-moirai ── moirai-executor
└── tyche-consus ── consus-zarr ── consus-core
```

`tyche-core` is `no_std + alloc` and depends only on Eunomia scalar law. It
owns:

- validated `Cow`-named parameter intervals and const-width spaces;
- counter-addressed deterministic streams;
- an affine-permutation random-access Latin hypercube;
- named studies and index-addressed fixed-array samples;
- a GAT model response seam and static response reducer;
- Welford/Chan moments with explicit denominator policy;
- squared-correlation screening with honest nomenclature;
- corrected split-conformal finite-sample calibration.

`tyche-moirai` borrows `HybridExecutor` and executes disjoint mutable output
chunks in a Moirai `SyncTask` scope. It does not create, globalize, or shut down
a runtime. Each trial writes exactly one caller-owned slot. Model errors remain
in those slots; scheduler failures use a distinct adapter error.

`tyche-consus` borrows a `consus_zarr::Store`, validates every artifact key
before the provider sees it, and delegates byte reads and writes without an
intermediate Tyche-owned copy. Persistence occurs only after trial dispatch,
in logical index order.

## Theorems and proof obligations

### Affine-permutation stratification theorem

Let `n > 0`, `gcd(a, n) = 1`, and `b` be arbitrary. Define
`pi(i) = a i + b (mod n)` on residues `0..n-1`.

**Claim.** `pi` is a bijection.

**Proof.** Suppose `pi(i) = pi(j)`. Then `a(i-j) = 0 (mod n)`. Since `a` is
invertible modulo `n`, multiplication by its inverse gives `i-j = 0 (mod n)`.
Both indices lie in one complete residue system, so `i = j`. A finite
injective self-map is bijective.

For `x_i = (pi(i)+u_i)/n` and `0 <= u_i < 1`,
`floor(n x_i) = pi(i)`. Therefore every stratum appears exactly once in every
dimension. Tests exhaust all strata for prime and composite sample counts.

### Interval-preservation theorem

Let finite `l < h` and `0 <= u < 1`. Because `h-l > 0`,
`0 <= u(h-l) < h-l`. Adding `l` yields
`l <= l+u(h-l) < h`.

Native floating arithmetic may round an extreme mapped point to `h`; Phase 0
tests bounded scientific ranges and documents the real-arithmetic contract
rather than claiming impossible universal IEEE exactness.

### Replay-invariance theorem

Every normalized coordinate is a pure function of
`(algorithm version, seed, sample index, dimension)`. The mapped parameter
sample is a pure function of that coordinate and immutable bounds. Each Moirai
trial writes the slot named by the same sample index.

**Claim.** If a model is deterministic for its parameter sample, scheduler
order cannot change the index-addressed response bits.

**Proof.** Scheduling does not appear in any stream key, parameter expression,
model input, or destination index. A permutation of trial execution therefore
permutes only evaluation time, not inputs or destinations. Every slot contains
the same result after the scope joins. Floating summaries then traverse slots
in ascending logical index, so worker count cannot change arithmetic
association.

### Welford invariant

After `n` observations, let the stored mean be `m_n` and centered sum be `M2_n`.
The update is:

`delta = x_n - m_(n-1)`

`m_n = m_(n-1) + delta/n`

`M2_n = M2_(n-1) + delta (x_n - m_n)`.

**Claim.** In exact arithmetic, `m_n = (1/n) sum x_i` and
`M2_n = sum (x_i-m_n)^2`.

**Proof.** The mean identity follows directly by substituting the induction
hypothesis. Expanding the old centered terms around the new mean makes the
linear cross term vanish because deviations around `m_(n-1)` sum to zero; the
remaining shift term plus the new observation reduces to
`delta(x_n-m_n)`. This is exactly the update above. The base case `n=1` has
mean `x_1` and centered sum zero.

Population variance is `M2_n/n`. Unbiased sample variance is `M2_n/(n-1)` for
`n >= 2`. Distinct zero-sized policies make the denominator a compile-time
choice and reject undefined counts.

### Correlation-screening bound

For centered vectors `x` and `y`, the reported value is

`r^2 = <x,y>^2 / (||x||^2 ||y||^2)`.

Cauchy-Schwarz gives `<x,y>^2 <= ||x||^2 ||y||^2`, hence
`0 <= r^2 <= 1`. Degenerate zero-variance inputs report zero. A final clamp
only contains floating rounding. The measure is screening, not a Sobol
decomposition; the API makes no interaction-effect claim.

### Split-conformal finite-sample guarantee

Given `n` exchangeable calibration nonconformity scores and one future score,
their rank among `n+1` exchangeable values is uniform under random tie
breaking. Selecting calibration order statistic

`k = ceil((n+1)(1-alpha))`,

capped at `n`, excludes at most an `alpha` fraction of possible ranks.
Therefore marginal coverage is at least `1-alpha`, subject to exchangeability
and the chosen nonconformity function. The implementation validates all scores
before sorting caller-owned storage in place.

### Artifact logical-completeness obligation

A future versioned study schema must write trial payloads in index order and
write its completion manifest last. Readers reject a run without that
manifest. This establishes logical completeness only. The current Consus
`Store` contract exposes no flush, fsync, or transaction, so Tyche makes no
crash-atomic durability claim.

## Representation and cost consequences

- Parameter and study names borrow through `Cow`; numeric samples never use
  `Cow`.
- Const parameter widths use fixed arrays and reject shape drift at compile
  time.
- `StudyModel::Response<'a>` may borrow; `ResponseReducer` consumes it within
  the evaluation scope.
- Designs, scalar types, model types, reducers, variance policies, and Moirai
  chunk widths are statically dispatched and monomorphized.
- Counter/distribution and variance markers are zero-sized.
- Core sample generation and online statistics allocate nothing.
- Moirai output storage is caller-owned; Consus serialization is an explicit
  boundary where bytes necessarily exist.

## Rejected alternatives

- Mutable RNG per worker: rejected because schedule changes stream assignment.
- Stored Latin-hypercube matrix: rejected because random access needs only
  `O(PARAMETERS)` coefficients.
- Tyche-owned thread pool: rejected because Moirai owns execution.
- Persistence from worker callbacks: rejected because Consus stores need
  ordered mutable access and some backends bridge async work internally.
- Umbrella Consus dependency in core: rejected because it couples study law to
  formats and runtime features.
- One variance default: rejected because real consumers intentionally use both
  population and sample conventions.
- Calling squared correlation Sobol sensitivity: rejected as mathematically
  false for interacting models.
- Dynamic experiment trait objects: rejected because known model/reducer types
  can borrow and monomorphize without boxing.

## Verification

- Exhaustive Latin-hypercube stratum uniqueness and generated bound properties.
- Bitwise same-seed replay in forward and reverse index order.
- Open-unit and standard-normal domain checks.
- Welford versus independent two-pass oracle; Chan merge identity.
- Population/sample denominator and singleton regressions.
- Affine correlation theorem.
- Corrected conformal upper-tail and in-place validation regressions.
- GAT borrowed-response and `Cow` pointer-identity assertions.
- ZST and zero-allocation core-loop assertions.
- Scoped Moirai output-index equivalence.
- Consus in-memory exact-byte round trip and traversal-safe keys.
- No-default-feature core check, warning-denied Clippy/rustdoc, doctests,
  examples, and supply-chain policy.
