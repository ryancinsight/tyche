# Tyche

Tyche is the Atlas owner for reproducible uncertainty studies. Phase 0
provides validated parameter spaces, deterministic random-access Latin
hypercube designs, index-addressed ensemble execution, online moments,
correlation-based sensitivity screening, finite-sample split-conformal
calibration, and provider adapters for Moirai and Consus.

The name refers to Tyche, the Greek goddess of fortune and chance.

## Boundary

Tyche owns:

- stable study identity, seeds, trial indices, and replay rules;
- experimental-design and distribution mechanics;
- ensemble statistics and explicitly named variance conventions;
- domain-neutral sensitivity and uncertainty calibration;
- the logical schema and keys of reproducible study artifacts.

Moirai owns scheduling, synchronization, and runtime lifecycle. Consus owns
files, object stores, formats, compression, and durability. Leto owns arrays,
Coeus and domain packages own model semantics, and integrators own their
physics. Tyche never constructs a hidden runtime, writes from worker
callbacks, or embeds a domain solver.

## Example

```rust
use core::num::NonZeroUsize;
use tyche::{
    Ensemble, LatinHypercube, Parameter, ParameterSpace, PopulationVariance,
    Seed, Study,
};

let space = ParameterSpace::new([
    Parameter::borrowed("flow", 1.0_f64, 3.0).expect("valid flow interval"),
    Parameter::borrowed("pressure", 10.0, 20.0).expect("valid pressure interval"),
])
.expect("unique parameter names");
let design = LatinHypercube::new(
    Seed::new(0x5459_4348_45),
    NonZeroUsize::new(64).expect("positive sample count"),
);
let study = Study::borrowed("pump sweep", space, design).expect("named study");

let mut responses = Vec::with_capacity(study.sample_count());
for index in 0..study.sample_count() {
    let sample = study.sample(index).expect("valid design index");
    responses.push(sample.values()[0] * sample.values()[1]);
}
let moments = Ensemble::new(&responses).moments().expect("non-empty ensemble");

assert_eq!(moments.count(), 64);
assert!(moments.variance::<PopulationVariance>().expect("defined") >= 0.0);
```

Production consumers use `tyche-moirai::MoiraiDispatch` with a borrowed
executor and caller-owned result slots. Persistence uses
`tyche-consus::ConsusArchive` over a borrowed `consus_zarr::Store` after
dispatch completes.

## Architecture

```text
crates/
├── tyche-core/
│   └── src/
│       ├── design/       # Cow-backed parameter identity and const-width spaces
│       ├── sampling/     # counter streams and random-access LHS
│       ├── study/        # named specification and fixed-width samples
│       ├── ensemble/     # GAT model seam and zero-copy response view
│       ├── statistics/   # Welford/Chan moments and correlation screening
│       └── uncertainty/  # corrected finite-sample conformal calibration
├── tyche-moirai/
│   └── src/              # scoped borrowed-executor adapter
├── tyche-consus/
│   └── src/              # validated keys and borrowed Store adapter
└── tyche/
    └── src/              # curated public facade
```

Every `lib.rs` and `mod.rs` is a manifest. Algorithms live in leaves.
`tyche-core` is `no_std + alloc`; it has no runtime or persistence dependency.
Moirai and Consus adapters depend inward on the core, never the reverse.

`Parameter`, `Study`, and `ArtifactKey` use `Cow<str>` so static metadata
borrows and runtime metadata owns through one API. Parameter and sample widths
are const generics. `StudyModel::Response<'a>` is a GAT that permits a response
to borrow model configuration or the current sample until a statically
dispatched `ResponseReducer` consumes it. The compiler monomorphizes designs,
models, reducers, scalar precision, parameter width, and Moirai chunk width;
there are no algorithm-path vtables.

`SplitMix64`, `StandardNormal`, `PopulationVariance`, and `SampleVariance` are
zero-sized policies. Latin hypercube sampling writes directly into fixed
arrays and stores only `O(PARAMETERS)` affine coefficients rather than an
`O(SAMPLES × PARAMETERS)` design matrix. Repeated core sampling and statistics
perform no allocation.

## Mathematical evidence

For sample count `n`, Tyche selects in each dimension a stride `a` coprime to
`n` and offset `b`. The map

`pi(i) = a i + b (mod n)`

is a permutation, so

`x_i = (pi(i) + u_i) / n`, with `0 <= u_i < 1`,

places exactly one point in every stratum. Counter-addressed jitter depends
only on `(seed, sample index, dimension)`, making replay independent of
execution order.

Welford's recurrence stores the exact arithmetic mean and centered sum of
squares after each prefix. Population variance divides by `n`; sample variance
divides by `n - 1`. The convention is a required zero-sized type parameter, so
singleton sample variance is a typed error rather than `NaN`.

Squared Pearson screening lies in `[0, 1]` by Cauchy-Schwarz. It is deliberately
not labelled Sobol: correlation screening cannot distinguish first-order from
total interaction effects. Split-conformal calibration uses the corrected
one-based rank

`ceil((n + 1)(1 - alpha))`,

capped at `n`; this is the finite-sample exchangeability correction, not the
reversed lower-tail percentile found in one legacy consumer.

Full assumptions, proofs, provider consequences, and rejected alternatives
are the single source of truth in
[ADR 0001](docs/adr/0001-reproducible-study-boundary.md).

## Phase 0 verification

```text
cargo fmt --all -- --check
cargo check -p tyche-core --no-default-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
cargo test --doc --workspace --all-features
cargo doc --workspace --no-deps --all-features
cargo run -p tyche --example reproducible_study
cargo deny check
```

The suite checks Latin-hypercube stratification, bounds and bitwise replay;
open-unit and standard-normal stream domains; generic `f32`/`f64` statistics;
Welford and Chan identities; explicit variance conventions; conformal rank;
GAT borrowing; `Cow` pointer identity; zero-sized policies; allocation-free
core loops; Moirai index preservation; Consus exact-byte round trips; and
artifact-key traversal rejection.

## Planned vertical increments

Phase 0 intentionally provides one complete design and one honestly named
sensitivity method. Subsequent increments are dependency ordered:

1. Add random-access Sobol, runtime-dimension views, categorical and weighted
   sampling, and versioned distribution golden vectors.
2. Replace Helios's private normal generator, CFDrs's duplicated LHS, and
   Kwavers's duplicated conformal/moment machinery after Tyche has a public
   remote revision.
3. Add reproducible bootstrap intervals, Morris screening, and true Saltelli
   first-order/total Sobol estimators with interaction oracles.
4. Add a versioned multi-output Consus study schema and manifest-last logical
   completeness. Crash durability waits for a Consus flush/transaction
   capability and will not be simulated in Tyche.

The detailed live gap inventory is in [`gap_audit.md`](gap_audit.md), and the
delivery order is in [`backlog.md`](backlog.md).

## License

Licensed under either the MIT License or Apache License 2.0.
