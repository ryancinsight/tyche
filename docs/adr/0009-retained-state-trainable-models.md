# ADR 0009: Retained-state trainable models

- Status: Accepted
- Date: 2026-11-24
- Class: [minor]
- Revised: 2026-11-24, the refit statement narrowed to what `BaggedModel::predict` provably does (calls only `TrainableModel::predict`, never `fit`), because no trait text excludes a `Fitted` that stores observations; evidence: `bagging.rs:100-108` and the fit counter at `tests/main/bagging.rs:118-120` (PR #144)

## Context

`StudyModel::evaluate` (`crates/tyche-core/src/ensemble/model.rs:4-22`) maps
one parameter sample to a response and has no training step. Ensemble bagging
fits a model on each bootstrap resample and later predicts from the fitted
states. The consumer model recorded in `gap_audit.md` retains no trained model
and perturbs an external predictor at prediction time, so the fitted members
are not a function of the resamples and the consumer cannot adopt provider
resampling. [ADR 0006](0006-release-scope-1-0.md) lists the seam in
M-uq-closure and records no contract for it.

## Decision

- `TrainableModel<T>` (`model.rs:72-100`): the implementor holds
  hyperparameters; `fit(&self, observations, resample)` returns the fitted
  state as an owned `Fitted` value, and `predict(&self, &Fitted, &Input)`
  reads it. `BaggedModel::predict` calls only `TrainableModel::predict`,
  never `fit` (`bagging.rs:100-108`), and the fit counter of the bagging test
  witnesses this for its model. The contract places no restriction on
  `Fitted`, so an instance-based model whose state is the observations stays
  admissible and may do its own work inside `predict`. Dispatch is static.
- `Bagging<A>` (`bagging.rs:19-74`) wraps a `Bootstrap<A>` and a nonzero
  member count. Member `r` fits the resample at replicate `r`, filled by
  `Bootstrap::fill_into`. Bagging adds no random stream: replay is governed by
  `A::VERSION`, which is `SplitMix64::VERSION` for the only
  `StreamAlgorithm` implementor at this revision. A consumer that persists
  bagging results records that version and never compares them with a former
  local modulo schedule.
- `BaggedModel` (`bagging.rs:78-109`) retains one `Fitted` per member in
  replicate order. `predict` returns the mean of the members' predictions,
  accumulated through `Moments`, and never refits.
- `BaggingError<E>` (`bagging.rs:115-125`, `#[non_exhaustive]`) separates an
  observation count that differs from the bootstrap population
  (`PopulationLength`) from a member failure (`Model`). The first failing
  member ends the fit.

## Rejected alternatives

- A score-only model whose prediction perturbs an external predictor: it
  retains no trained state, so no resample reaches a prediction.
- Model-held fit state behind `&self` interior mutability: one model object
  would hold one fit, while bagging needs one state per member; the state
  must be a value that `fit` returns.
- Bagging that draws its own resample indices: a second schedule would need
  its own replay version beside the bootstrap's.

## Consequences

- `BaggedModel` holds members times the size of `Fitted` until it drops, plus
  one resample buffer of `resample_size` indices during `fit`.
- `BaggedModel` borrows the model (`&'model M`), so the model outlives it.
- The mean is the only aggregation provided.
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
