# Tyche

Tyche is the Atlas owner for reproducible uncertainty studies. Phase 0
provides validated parameter spaces, deterministic random-access Latin
hypercube designs, index-addressed ensemble execution, online moments,
correlation screening, finite-sample split-conformal calibration, and provider
adapters for Moirai and Consus.

## Boundary

Tyche owns study identity, seed and replay laws, sampling designs, ensemble
statistics, domain-neutral sensitivity, calibration, and logical artifact
keys. Moirai owns scheduling and runtime lifecycle. Consus owns stores,
formats, compression, and durability. Domain packages own their physics and
model semantics.

## Example

```rust
use core::num::NonZeroU32;
use tyche::{
    Ensemble, LatinHypercube, Parameter, ParameterSpace, PopulationVariance,
    Seed, Study,
};

let space = ParameterSpace::new([
    Parameter::borrowed("flow", 1.0_f64, 3.0).expect("valid"),
    Parameter::borrowed("pressure", 10.0, 20.0).expect("valid"),
])
.expect("unique");
let design = LatinHypercube::new(
    Seed::new(0x5459_4348_45),
    NonZeroU32::new(64).expect("positive"),
);
let study = Study::borrowed("pump sweep", space, design).expect("named");
let responses: Vec<_> = (0..study.sample_count())
    .map(|index| {
        let sample = study.sample(index).expect("valid");
        sample.values()[0] * sample.values()[1]
    })
    .collect();
let moments = Ensemble::new(&responses).moments().expect("non-empty");
assert!(moments.variance::<PopulationVariance>().expect("defined") >= 0.0);
```

## Architecture

```text
crates/
├── tyche-core/
│   └── src/
│       ├── design/       # Cow metadata and const-width spaces
│       ├── sampling/     # counter streams and random-access LHS
│       ├── study/        # named specifications and fixed arrays
│       ├── ensemble/     # GAT model seam and borrowed views
│       ├── statistics/   # Welford/Chan and screening
│       └── uncertainty/  # conformal calibration
├── tyche-moirai/         # borrowed scoped execution
├── tyche-consus/         # validated Store adaptation
└── tyche/                # curated public facade
```

`tyche-core` is `no_std + alloc` and has no runtime or persistence dependency.
`Parameter`, `Study`, and `ArtifactKey` use `Cow<str>` for one borrowed/owned
API. Numeric widths are const generics. `StudyModel::Response<'a>` is a GAT;
the statically dispatched reducer consumes it before its borrow ends. Designs,
models, reducers, scalar precision, variance policy, and Moirai chunk width
monomorphize without algorithm-path vtables.

`SplitMix64`, `StandardNormal`, `PopulationVariance`, and `SampleVariance` are
zero-sized. The Latin hypercube stores `O(PARAMETERS)` coefficients instead of
an `O(SAMPLES × PARAMETERS)` matrix. Repeated core sampling and statistics
allocate nothing. Public design failures remain typed across `Study` and the
Moirai adapter; see
[ADR 0002](docs/adr/0002-typed-design-errors.md).

## Mathematical evidence

For sample count `n`, each dimension uses stride `a` coprime to `n` and offset
`b`. The affine map `pi(i)=a*i+b (mod n)` is a permutation, hence
`x_i=(pi(i)+u_i)/n`, `0<=u_i<1`, places one point in every stratum.
Counter-addressed jitter depends only on `(seed,index,dimension)`, so Moirai
scheduling cannot change inputs or output slots.

Welford's recurrence stores the mean and centered sum. Population variance
divides by `n`; sample variance divides by `n-1`. The required zero-sized policy
makes singleton sample variance a typed error instead of `NaN`.

Squared Pearson screening lies in `[0,1]` by Cauchy-Schwarz and is deliberately
not called Sobol. Split-conformal calibration uses corrected rank
`ceil((n+1)(1-alpha))`, capped at `n`. Rank arithmetic remains in the score
scalar's native precision; already sorted scores use an allocation-free
borrowed calibration path.

Proofs and consequences are the SSOT in
[ADR 0001](docs/adr/0001-reproducible-study-boundary.md).

## Verification

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

## Planned vertical increments

1. Add random-access Sobol, runtime-dimension views, categorical and weighted
   sampling, and versioned distribution vectors.
2. Consumer integration is in delivery: [Helios PR 10] replaces its normal
   generator, [CFDrs PR 299] replaces its LHS, and [Kwavers PR 298] replaces
   its conformal, moment, and mislabeled sensitivity implementations.
3. Add deterministic bootstrap, Morris, and true Saltelli Sobol estimators.
4. Add a versioned Consus study schema with manifest-last logical completeness.
   Crash durability waits for a Consus transaction capability.

See [`gap_audit.md`](gap_audit.md) and [`backlog.md`](backlog.md).

[Helios PR 10]: https://github.com/ryancinsight/helios/pull/10
[CFDrs PR 299]: https://github.com/ryancinsight/CFDrs/pull/299
[Kwavers PR 298]: https://github.com/ryancinsight/kwavers/pull/298
