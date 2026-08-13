# Sensitivity Screening

Tyche provides three global screening surfaces. All of them accumulate
online and report typed, clamped summaries, so a study can rank parameter
influence without storing the full response history.

## Squared-correlation screening

`CorrelationScreening<T, PARAMETERS, OUTPUTS = 1>` is an online squared-Pearson
accumulator. The default single-output form accepts `(parameters, response)`
pairs through `update`; multi-output studies use `update_outputs` with one
response array per observation. `report` returns one independent correlation
vector per output, clamped to the unit interval. Two observations are required
for a defined correlation.

The following is a focused, non-standalone API fragment:

```rust,ignore
use tyche_core::statistics::CorrelationScreening;

let mut screening = CorrelationScreening::<f64, 2>::new();
for sample in sweep {
    screening.update(&[sample.x0, sample.x1], sample.response);
}
let report = screening.report()?;
let influence = report.squared_correlations();
```

## Morris elementary effects

Morris screening perturbs one parameter at a time along repeated
trajectories. A trajectory for `PARAMETERS` parameters walks
`PARAMETERS + 1` design points: a shared start point followed by one step
per parameter, where each step changes exactly one coordinate by `±delta`
and the response difference divided by `delta` is that parameter's
elementary effect.

`ElementaryEffects::from_steps` reduces one trajectory to its effect vector
and enforces the trajectory contract: `perturbed[step]` must name every
parameter exactly once (a permutation of `0..PARAMETERS`), and `delta` must
be strictly positive. Violations are typed as
`ElementaryEffectsError::{OutOfRangeParameter, DuplicateParameter,
NonPositiveStep}`.

`MorrisScreening` accumulates one effect vector per trajectory and reports,
per parameter and output, the mean `mu`, the mean absolute `mu_star`, and the
standard deviation `sigma`. `mu_star` ranks influence; a large `sigma` relative
to `mu_star` marks a parameter whose effect is nonlinear or interaction-driven.
The default single-output form uses `update`; multi-output studies use
`update_outputs`. Two effects per parameter are required so `sigma` is defined.

```rust,ignore
use tyche_core::statistics::{ElementaryEffects, MorrisScreening};

let mut screening = MorrisScreening::<f64, 2>::new();
for trajectory in trajectories {
    let effects = ElementaryEffects::from_steps(
        &trajectory.perturbed, trajectory.start_response,
        &trajectory.step_responses, delta,
    )?;
    screening.update(effects.effects());
}
let report = screening.report()?;
let ranking = report.mu_star();
```

## Saltelli Sobol' indices

The Saltelli A/B/`A_i^B` scheme draws `N` rows of an independent matrix `A`
and `N` rows of an independent matrix `B`; for each parameter `i` it also
evaluates the matrix `A_i^B` that agrees with `A` except that column `i`
comes from `B`. With `V` the sample variance of the `f(A)` responses, the
first-order estimate is

`S_i = sum(f(B)(f(A_i^B) - f(A))) / (N V)`

and the total-order estimate is

`S_Ti = sum((f(A) - f(A_i^B))^2) / (2 N V)`.

`SobolIndices::update(base, independent, recombined)` folds in one row
triple per sample — `base` is `f(A)`, `independent` is `f(B)`, and
`recombined[i]` is `f(A_i^B)`. `SobolReport` exposes `sample_count`,
`first_order` (`S_i`), and `total_order` (`S_Ti`). Two rows are required so
the `A` variance is defined, and finite-sample estimates are clamped to the
unit interval exactly like the squared-correlation screening. The default form
is scalar-output; `SobolIndices<T, PARAMETERS, OUTPUTS>` and
`update_outputs` retain one pair of index vectors per output without storing
the response history.

For a two-output study, the output axis is explicit:

```rust,ignore
use tyche_core::statistics::SobolIndices;

let mut estimator = SobolIndices::<f64, 2, 2>::new();
estimator.update_outputs(
    &[f_a0, f_a1],
    &[f_b0, f_b1],
    &[[f_a0_b0, f_a0_b1], [f_a1_b0, f_a1_b1]],
);
let report = estimator.report()?;
let first_order_by_output = report.first_order_by_output();
```

> **Independent A and B are required.** Consecutive points of one
> low-discrepancy sequence are far too correlated to serve as both matrices;
> draw A and B from distinct deterministic stream domains (for example two
> `UserDomain`-tagged `Counter` streams or a scrambled design), exactly as
> the integration tests do.

```rust,ignore
use tyche_core::statistics::SobolIndices;

let mut estimator = SobolIndices::<f64, 2>::new();
for row in rows {
    estimator.update(f(&a[row]), f(&b[row]), &[
        f(&a_with_b_column0),
        f(&a_with_b_column1),
    ]);
}
let report = estimator.report()?;
let first_order = report.first_order();
let total_order = report.total_order();
```
