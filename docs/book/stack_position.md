# Position in the Stack

Tyche is the reproducible uncertainty-study foundation of the Atlas stack.
It owns **study identity**, **parameter spaces**, **random-access
experimental designs**, **ensemble summaries**, **sensitivity screening**,
and **conformal calibration**. Execution and persistence are owned elsewhere
on purpose: Moirai owns execution, and Consus owns persistence.

## Ownership boundary

- **Tyche** provides validated `Parameter`/`ParameterSpace` contracts,
  counter-addressed streams and deterministic designs (`LatinHypercube`,
  `Sobol`), `Bootstrap` resampling, `Moments`/`CorrelationScreening`/
  Morris/Saltelli sensitivity surfaces, and `ConformalCalibrator`
  prediction intervals — all `no_std`-clean and allocation-conscious.
- **Moirai** owns the execution contract; `tyche-moirai` is a dependency-
  inverted adapter that dispatches Tyche `Design` implementations and study
  models, validated by `dispatch_contract` tests.
- **Consus** owns persistence; `tyche-consus::ConsusArchive` is a
  storage-only round-trip adapter. The versioned study schema and manifest
  semantics remain a Tyche-owned open item and are intentionally **not**
  duplicated into Consus storage policy.

## Consumers

Tyche's sampling and statistics seams are consumed without local copies:

- Helios uses `tyche_core::StandardNormal` in imaging noise.
- CFDrs routes design-space sampling through Tyche's sampling module.
- Kwavers uses `Bootstrap::<SplitMix64>` for elastography percentile-
  bootstrap index generation; percentile interpolation stays consumer-owned.

Adaptive estimator breadth (Morris and Saltelli Sobol' indices) now closes
the previously deferred screening increments; the versioned Consus study
schema and the score-only ensemble model remain explicitly open Tyche-owned
follow-ups, and the crates.io publication sequence is an external release
gate: `tyche-core` is `publish = true` with a tag-gated Trusted Publishing
workflow, while the workspace facade and adapter crates stay `publish =
false` until dependency-ordered registry publication is authorized.
