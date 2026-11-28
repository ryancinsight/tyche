# ADR 0009: Retained-state trainable models

- Status: Accepted
- Date: 2026-11-24
- Class: [minor]
- Revised: 2026-11-24, the refit statement narrowed to what `BaggedModel::predict` provably does (calls only `TrainableModel::predict`, never `fit`), because no trait text excludes a `Fitted` that stores observations; evidence: `bagging.rs:115-123` and the fit counter at `tests/main/bagging.rs:127-131` (PR #144)
- Revised: 2026-11-24, `BaggingError::ResampleAllocation` and `MemberAllocation` added: `Bagging::fit` reserves the resample buffer and the member storage with `Vec::try_reserve_exact` before any member fits, so a refused allocation returns an error where `vec!` panicked and the growing member `Vec` aborted; evidence: `fit_rejects_an_unallocatable_resample_with_a_typed_error` and `fit_rejects_unallocatable_member_storage_with_a_typed_error` in `tests/main/bagging.rs`.

- Revised: 2026-11-27, three consumer questions decided (multi-output, member spread, `RealField` bound), each as the recommended option and a veto point; evidence: Kwavers `34fa3f47` `ensemble_methods/mod.rs:42-46,165-176`, `ensemble_methods/model.rs:21`, `predictor.rs:20`, `bagging.rs:44-51,99-123` and `statistics/moments.rs` (PR #158)

## Context

`StudyModel::evaluate` (`crates/tyche-core/src/ensemble/model.rs:4-22`) maps
one parameter sample to a response and has no training step. Ensemble bagging
fits a model on each bootstrap resample and later predicts from the fitted
states. The consumer model recorded in `gap_audit.md` retains no trained model
and perturbs an external predictor at prediction time, so the fitted members
are not a function of the resamples and the consumer cannot adopt provider
resampling. [ADR 0006](0006-release-scope-1-0.md) lists the seam in
M-uq-closure and records no contract for it.

Three questions remained open. The consumer predicts an `Array2<f32>` and
reports a per-element, weighted variance across its ensemble members
(Kwavers `34fa3f47`, `ensemble_methods/mod.rs:42-46,165-176`), while
`TrainableModel::predict` returns one `T` (`model.rs:101`) and
`BaggedModel::predict` returns only the mean (`bagging.rs:115-123`). Separately,
`fitted_members` and `predict` require `T: RealField` (`bagging.rs:99`) while
`Bagging::fit` requires nothing of `T` (`bagging.rs:44-51`). At that revision
the consumer's member weights default to 1.0 (`ensemble_methods/model.rs:21`)
and change only through its public `update_weights` (`mod.rs:243-262`), its
predictor maps a whole input matrix to a whole output matrix per call
(`predictor.rs:20`), and no consumer type implements `TrainableModel`.

## Decision

- `TrainableModel<T>` (`model.rs:74-102`): the implementor holds
  hyperparameters; `fit(&self, observations, resample)` returns the fitted
  state as an owned `Fitted` value, and `predict(&self, &Fitted, &Input)`
  reads it. `BaggedModel::predict` calls only `TrainableModel::predict`,
  never `fit` (`bagging.rs:115-123`), and the fit counter of the bagging test
  witnesses this for its model. The contract places no restriction on
  `Fitted`, so an instance-based model whose state is the observations stays
  admissible and may do its own work inside `predict`. Dispatch is static.
- `Bagging<A>` (`bagging.rs:19-89`) wraps a `Bootstrap<A>` and a nonzero
  member count. Member `r` fits the resample at replicate `r`, filled by
  `Bootstrap::fill_into`. Bagging adds no random stream: replay is governed by
  `A::VERSION`, which is `SplitMix64::VERSION` for the only
  `StreamAlgorithm` implementor at this revision. A consumer that persists
  bagging results records that version and never compares them with a former
  local modulo schedule.
- `BaggedModel` (`bagging.rs:93-124`) retains one `Fitted` per member in
  replicate order. `predict` returns the mean of the members' predictions,
  accumulated through `Moments`, and never refits.
- `BaggingError<E>` (`bagging.rs:130-150`, `#[non_exhaustive]`) separates an
  observation count that differs from the bootstrap population
  (`PopulationLength`), a resample buffer or the storage for the member
  states that cannot be allocated (`ResampleAllocation`, `MemberAllocation`:
  one variant per buffer, so each `Display` names the failing allocation;
  both reserved with `Vec::try_reserve_exact` before any member fits), and a
  member failure (`Model`). The first failing member ends the fit.
- Multi-output (veto point 1): `TrainableModel<T>::predict` keeps one scalar
  `T` and the provider adds no output-shape type. A consumer with an array
  response addresses one element per call through `Input`, an unconstrained
  `?Sized` type, so one retained `Fitted` serves every element. The mean and
  the variance are elementwise statistics: the mean over members of element
  `(i, j)` equals element `(i, j)` of the mean array, so the scalar seam
  loses neither. The consumer assembles its `Array2` from those calls. The
  first overturning trigger below holds in form at the cited revision (the
  predictor evaluates the whole matrix per call) but is unmeasured, and no
  consumer type implements the trait, so a vector `Output` has neither a
  measurement nor a second implementor; this is the weakest of the three
  decisions.
- Member spread (veto point 2): `BaggedModel` gains
  `predict_moments(&Input) -> Result<Moments<T>, M::Error>`, the accumulator
  `predict` already builds, and `predict` becomes its mean. The caller picks
  the divisor through `Moments::variance::<P>` (`PopulationVariance` or
  `SampleVariance`), so the provider fixes no variance convention. The spread
  is that of the member predictions about their mean, unweighted, with no
  noise term, so it reproduces the consumer's statistic at default weights
  only. A consumer that keeps non-uniform weights computes its weighted
  reduction from `fitted_members` and `TrainableModel::predict`, and whether
  that policy persists under provider bagging is the consumer owner's
  decision, not made here; percentiles stay the consumer's. Filed as
  TYCHE-BAGGING-SPREAD-001.
- Bound (veto point 3): `T: RealField` is required exactly where a mean is
  formed, so `predict` and `predict_moments` keep it. `Bagging::fit` forms no
  mean and stays unbounded, and `fitted_members`, which only reads the
  retained `Vec<M::Fitted>`, moves to an impl block without the bound, so
  every fitted bag is readable. The `RealField` bound sits at the use site,
  never on the `Bagging::fit` signature or the `BaggedModel` definition, which
  keeps `M: TrainableModel<T>` only to name `M::Fitted` (`bagging.rs:93`).
  Filed as TYCHE-BAGGING-BOUND-001.

## Rejected alternatives

- A score-only model whose prediction perturbs an external predictor: it
  retains no trained state, so no resample reaches a prediction.
- Model-held fit state behind `&self` interior mutability: one model object
  would hold one fit, while bagging needs one state per member; the state
  must be a value that `fit` returns.
- Bagging that draws its own resample indices: a second schedule would need
  its own replay version beside the bootstrap's.
- Multi-output through an `Output` associated type with its own accumulation
  trait: the only aggregations with a caller are the elementwise mean and
  variance, which commute with element addressing, so the trait would have
  one implementor (no speculative seam). `predict` into a caller `&mut [T]`
  adds a length contract and a typed error, and per-slot accumulators need
  storage the provider would own or the caller supplies.
- Member spread left to the consumer through `fitted_members` and
  `TrainableModel::predict`: both are public, so it is possible, but every
  consumer repeats the member loop and the Welford accumulation, and the
  mean and the spread would come from different accumulators. Returning a
  `Vec<T>` of member predictions allocates per call and exposes more than the
  spread needs.
- `T: RealField` on `Bagging::fit` or on the `BaggedModel` definition: it
  rejects a fit-only use with a non-scalar `T` and bounds a type that reads
  nothing scalar.

## Consequences

- `BaggedModel` holds members times the size of `Fitted` until it drops, plus
  one resample buffer of `resample_size` indices during `fit`.
- `BaggedModel` borrows the model (`&'model M`), so the model outlives it.
- The mean is the only aggregation provided until TYCHE-BAGGING-SPREAD-001
  adds the moments accumulator; the three decisions above change no code.
- Evidence: `crates/tyche-core/tests/main/bagging.rs` fits a least-squares
  model on integer data for `f32`, `f64`, `F32` and `F64`, compares each member
  with an independently derived slope, bounds the bagged mean by a derived
  rounding bound, and asserts a fit counter equal to the member count after
  both predictions. Delivered by PR 142.

## Overturning evidence

- A consumer model whose training is not a function of an observation slice
  and a resample index list, such as a weighted or streaming fit, reopens the
  `fit` signature.
- A measured `Fitted` footprint that makes members times its size
  untenable for a consumer reopens retention of every member.
- Multi-output: a measured `predict` cost per element that dominates, for a
  predictor that evaluates the whole array per call, or a statistic that
  couples outputs (a covariance across elements) reopens a vector `Output`.
- Member spread: a consumer that needs weighted spread or quantiles of the
  member predictions reopens exposing the member predictions themselves; the
  cited consumer meets the weighted case once `update_weights` is called.
- `RealField` bound: an aggregation over a non-scalar `T` with a defined mean
  reopens the bound on `predict`.
