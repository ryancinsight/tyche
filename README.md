# Tyche

Tyche is the Atlas owner for reproducible uncertainty studies. Phase 0
provides validated parameter spaces, deterministic random-access Latin
hypercube and Sobol designs, fixed and runtime dimension selection,
uniform and weighted finite-distribution sampling, support-checked discrete
importance ratios, index-addressed ensemble execution, online moments,
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
    Seed, SplitMix64, Study,
};

let space = ParameterSpace::new([
    Parameter::borrowed("flow", 1.0_f64, 3.0).expect("valid"),
    Parameter::borrowed("pressure", 10.0, 20.0).expect("valid"),
])
.expect("unique");
let design = LatinHypercube::<2, SplitMix64>::new(
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
│       ├── sampling/     # typed streams and fixed/runtime designs
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
API. Discrete mass tables use `Cow<[T]>`, retaining borrowed inputs without
normalization copies while accepting owned storage through the same contract.
Numeric widths are const generics. `StudyModel::Response<'a>` is a GAT;
the statically dispatched reducer consumes it before its borrow ends. Designs,
models, reducers, scalar precision, variance policy, and Moirai chunk width
monomorphize without algorithm-path vtables.

`SplitMix64`, typed `Counter` domains, `StandardNormal`,
`PopulationVariance`, and `SampleVariance` are zero-sized. Stream algorithm
selection has no default: the type and nonzero version identify the bitwise
replay contract. Native `f32` and `f64` unit conversion avoids hidden
widen/narrow arithmetic. The Latin hypercube stores `O(PARAMETERS)`
coefficients instead of an `O(SAMPLES × PARAMETERS)` matrix. Repeated core
sampling and statistics allocate nothing. Fixed and runtime Sobol designs
share one const-generic kernel; a row-major runtime fill validates and
dispatches once without nested point storage. Public design failures remain
typed across `Study` and the Moirai adapter; see [ADR 0002], [ADR 0003], and
[ADR 0004].

## Mathematical evidence

For sample count `n`, each dimension uses stride `a` coprime to `n` and offset
`b`. The affine map `pi(i)=a*i+b (mod n)` is a permutation, hence
`x_i=(pi(i)+u_i)/n`, `0<=u_i<1`, places one point in every stratum.
Counter-addressed jitter depends only on `(seed,index,dimension)`, so Moirai
scheduling cannot change inputs or output slots.

Typed domains separate LHS stride, offset, jitter, and normal transform words.
The explicit Mix13 schedule is pinned by raw-word, native-unit, normal, and LHS
known-answer vectors. ADR 0003 proves equal-coordinate domain separation and
records the controlled Criterion comparison against the untyped schedule.

Sobol point `n` is the XOR of direction numbers selected by the set bits of
its Gray code. Consecutive Gray codes toggle the same direction selected by the
sequential recurrence, proving random-access equivalence. Origin-aligned
power-of-two prefixes stratify every one-dimensional dyadic projection; a
fixed digital shift permutes those strata. ADR 0004 states the proof limits,
exact vectors, differential oracle, memory contract, and controlled benchmark.

For a nonzero category count `s`, categorical sampling multiplies a uniform
`u64` word `x` by `s`, rejects when the product's low half lies below
`2^64 mod s`, and returns the high half. The accepted source-space cardinality
is divisible by `s`, so every category has the same number of preimages. The
counter retry coordinate is a permutation of all `u64` words, so rejection
terminates for each address.

Weighted sampling returns the first category whose cumulative mass exceeds
the native-precision uniform threshold. In ideal real arithmetic the interval
length is exactly the category mass; the implementation states the finite
`f32`/`f64` unit-grid quantization rather than widening silently. For target
`p`, proposal `q`, and response `f`, the enforced support law gives
`E_q[f(I) p_I/q_I] = sum_i p_i f_i`. [ADR 0005] records the reference theorems,
proof obligations, finite-precision limit, and executable evidence.

Welford's recurrence stores the mean and centered sum. Population variance
divides by `n`; sample variance divides by `n-1`. The required zero-sized policy
makes singleton sample variance a typed error instead of `NaN`.

Squared Pearson screening lies in `[0,1]` by Cauchy-Schwarz and is deliberately
not called Sobol. Split-conformal calibration uses corrected rank
`ceil((n+1)(1-alpha))`, capped at `n`. Rank arithmetic remains in the score
scalar's native precision; already sorted scores use an allocation-free
borrowed calibration path.

Proofs and consequences are owned by the boundary and algorithm ADRs,
beginning with [ADR 0001](docs/adr/0001-reproducible-study-boundary.md).

[ADR 0002]: docs/adr/0002-typed-design-errors.md
[ADR 0003]: docs/adr/0003-domain-separated-counter-schedule.md
[ADR 0004]: docs/adr/0004-random-access-sobol.md
[ADR 0005]: docs/adr/0005-discrete-importance-sampling.md

## Verification

```text
cargo fmt --all -- --check
cargo check -p tyche-core --no-default-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
cargo test --doc --workspace --all-features
cargo doc --workspace --no-deps --all-features
cargo run -p tyche --example reproducible_study
cargo bench -p tyche-core --bench counter_sampling
cargo deny check
```

## Roadmap

1. Existing consumer integration is delivered: merged [Helios PR 10] replaces
   its normal generator, merged [CFDrs PR 299] replaces its LHS, merged
   [Kwavers PR 298] replaces its conformal, moment, and mislabeled sensitivity
   implementations, and merged [Kwavers PR 304] replaces fixed collocation LHS
   and Sobol generation. Geometry mappings remain Kwavers-owned.
2. Add deterministic bootstrap resampling and migrate Kwavers percentile-mean
   and ensemble-bagging index generation to that provider contract.
3. Add genuine Morris and Saltelli Sobol estimators plus multi-output reports.
4. Add a versioned Consus study schema with manifest-last logical completeness.
   Crash durability waits for a Consus transaction capability.

See [`gap_audit.md`](gap_audit.md) and [`backlog.md`](backlog.md).

[Helios PR 10]: https://github.com/ryancinsight/helios/pull/10
[CFDrs PR 299]: https://github.com/ryancinsight/CFDrs/pull/299
[Kwavers PR 298]: https://github.com/ryancinsight/kwavers/pull/298
[Kwavers PR 304]: https://github.com/ryancinsight/kwavers/pull/304
