# ADR 0009: Retained-state trainable models

- Status: Accepted
- Date: 2026-11-24
- Class: [minor]
- Revised: 2026-11-24, the refit statement narrowed to what `BaggedModel::predict` provably does (calls only `TrainableModel::predict`, never `fit`), because no trait text excludes a `Fitted` that stores observations; evidence: `bagging.rs:115-123` and the fit counter at `tests/main/bagging.rs:127-131` (PR #144)
- Revised: 2026-11-24, `BaggingError::ResampleAllocation` and `MemberAllocation` added: `Bagging::fit` reserves the resample buffer and the member storage with `Vec::try_reserve_exact` before any member fits, so a refused allocation returns an error where `vec!` panicked and the growing member `Vec` aborted; evidence: `fit_rejects_an_unallocatable_resample_with_a_typed_error` and `fit_rejects_unallocatable_member_storage_with_a_typed_error` in `tests/main/bagging.rs`.
- Revised: 2026-11-28, multi-output, member predictions and the `RealField` bound decided, because a consumer adopting bagging needs each answered and the cited consumer holds statistics the scalar seam cannot produce; evidence: Kwavers `711b3f26` `ensemble_methods/mod.rs:42-46,165-239,245-264`, `ensemble_methods/model.rs:21`, `uncertainty/predictor.rs:20`, and `bagging.rs:44-51,93-123` (PR #158)

## Context

`StudyModel::evaluate` (`crates/tyche-core/src/ensemble/model.rs:4-22`) maps
one parameter sample to a response and has no training step. Ensemble bagging
fits a model on each bootstrap resample and later predicts from the fitted
states. The consumer model recorded in `gap_audit.md` retains no trained model
and perturbs an external predictor at prediction time, so the fitted members
are not a function of the resamples and the consumer cannot adopt provider
resampling. [ADR 0006](0006-release-scope-1-0.md) lists the seam in
M-uq-closure and records no contract for it.

Three questions remained open. At Kwavers `711b3f26` (the cited files are
unchanged since `34fa3f47`) the consumer's members predict `Array2<f32>`
(`ensemble_methods/mod.rs:42-46`), and it reports a per-element weighted
variance (`:165-176`) and a diversity score, the mean pairwise Frobenius
distance over whole member arrays (`:184-185`, `:196-218`), through the public
`get_ensemble_result(predictions, weights)` (`:224-239`). Member weights
default to 1.0 (`ensemble_methods/model.rs:21`) and arrive through
`update_weights` (`:245-264`) or the caller's `weights`. Its predictor maps a
whole input matrix to a whole output matrix per call
(`uncertainty/predictor.rs:20`), and no consumer type implements
`TrainableModel`. Here `predict` returns one `T` (`model.rs:101`),
`BaggedModel::predict` only the mean (`bagging.rs:115-123`), and
`fitted_members` and `predict` require `T: RealField` (`bagging.rs:99`) while
`Bagging::fit` requires nothing of `T` (`bagging.rs:44-51`).

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
  response addresses one element per call through `Input` (`?Sized`,
  unconstrained), so one retained `Fitted` serves every element and every
  member's prediction of every element stays reachable. Mean and variance are
  elementwise, so they commute with that addressing. The consumer's diversity
  score couples all elements and does not: member sets `(0,0),(1,1),(2,2)` and
  `(0,1),(1,0),(2,2)` share per-element mean 1 and population variance 2/3 but
  have diversities 1.885618 and 1.962117. That is the coupled-statistic
  trigger below, met in form; the scalar seam stays, and the consumer rebuilds
  such statistics from each element's member predictions, at one `predict`
  call per element that a whole-array predictor cannot share. This is the
  weakest decision.
- Member predictions (veto point 2): `BaggedModel` will expose each member's
  prediction for an input as a borrowing iterator of `Result<T, M::Error>`,
  and `predict` stays the mean over it. The iterator allocates nothing, keeps
  each `Fitted` paired with the model that fitted it (the bag borrows that
  model, `bagging.rs:93`), and serves every reduction the consumer computes:
  spread through the public `Moments` and the caller's variance policy,
  weighted variance, quantiles and diversity. Whether the consumer keeps
  non-uniform weights under provider bagging is its owner's call.
- Bound (veto point 3): `T: RealField` is required where a mean is formed, so
  `predict` keeps it and `Bagging::fit`, which forms none, stays unbounded.
  `fitted_members` and the member-prediction iterator read only retained
  states and the model, so they will sit in an impl block without the bound
  and every fitted bag is readable. The bound sits at the use site, not on
  `fit` or the definition, whose `M: TrainableModel<T>` only names `M::Fitted`
  (`bagging.rs:93`).

## Rejected alternatives

- A score-only model whose prediction perturbs an external predictor: it
  retains no trained state, so no resample reaches a prediction.
- Model-held fit state behind `&self` interior mutability: one model object
  would hold one fit, while bagging needs one state per member; the state
  must be a value that `fit` returns.
- Bagging that draws its own resample indices: a second schedule would need
  its own replay version beside the bootstrap's.
- Multi-output through an `Output` associated type with its own accumulation
  trait, or `predict` into a caller `&mut [T]`. Strongest: a whole-array
  predictor then evaluates once per member, not once per element. Rejected
  because statistics that couple elements (diversity) or weight members stay
  consumer policy, so the accumulation trait would have one implementor (no
  speculative seam), and `&mut [T]` adds a length contract, a typed error and
  per-slot storage.
- `predict_moments` returning `Moments<T>`. Strongest for spread: it serves
  the unweighted spread, but not the consumer's weights or diversity, and it
  equals the iterator folded into `Moments` in three lines, so it waits for a
  second consumer that repeats the fold. Leaving members to the consumer
  through `fitted_members` and `TrainableModel::predict` allows pairing a state
  with a model other than the one that fit it, and a `Vec<T>` of member
  predictions allocates per call.
- `T: RealField` on `Bagging::fit` or on the `BaggedModel` definition: it
  rejects a fit-only use with a non-scalar `T` and bounds a type that reads
  nothing scalar.

## Consequences

- `BaggedModel` holds members times the size of `Fitted` until it drops, plus
  one resample buffer of `resample_size` indices during `fit`.
- `BaggedModel` borrows the model (`&'model M`), so the model outlives it.
- The mean is the only aggregation provided; the provider owns no weighted or
  cross-element statistic, so a consumer that reports one (Kwavers
  `ensemble_methods/mod.rs:165-218`) rebuilds it from each element's member
  predictions.
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
- Multi-output: a statistic that couples outputs, as the cited consumer's
  diversity does in form, reopens a vector `Output`; it stays scalar here
  because that statistic rebuilds from per-element member predictions. A call
  counter on a consumer's predictor showing more than one whole-array
  evaluation per member per bagged prediction also reopens it; none exists.
- Member predictions: a second consumer folding the iterator into a spread
  with `Moments` the same way reopens `predict_moments`; a consumer keeping
  weights (`update_weights` or `weights`, as the cited one can) reopens them.
- `RealField` bound: an aggregation over a non-scalar `T` with a defined mean
  reopens the bound on `predict`.
