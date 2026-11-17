# Tyche implementation backlog

<a id="tyche-sobol-centered-003"></a>

## TYCHE-SOBOL-CENTERED-003 — Centered Sobol variance [patch] — todo

- Milestone: M-uq-closure; proposed patch version 0.4.1, not release approval.
- Outcome: offset responses do not lose influence through raw-moment cancellation.
- Scope: Sobol accumulation, analytical regressions, sensitivity documentation.
- Basis: `a14ab1ea`; priority: correctness; needs: none
- Acceptance: native binary32/binary64 offset, population, constant-output and
  allocation oracles pass; binary32/binary64 final-division underflow and
  denominator-overflow controls pass; complete applicable gates and exact-head independent
  review pass. The current correlation/provider/release hold remains unchanged.
- Evidence: unchanged production code fails the exact shifted-response oracle
  with zero rather than unit sensitivity; the centered candidate passes it.
  Full qualification and delivery remain pending.

<a id="tyche-correlation-range-002"></a>

## TYCHE-CORRELATION-RANGE-002 — Correlation range release evidence [major] — blocked

- **Milestone:** M-uq-closure (planned version 0.5.0, the release this item records).
- Outcome: 0.5.0 bump, correlation-repair ADR, provider hold, midpoint tests and
  `report` evidence that the repair dropped from released claim `refs/pull/117/head`.
- Acceptance: ADR 0007 (0006 is the 1.0 release scope) is Accepted, indexed and
  records the provider hold; a midpoint-index test passes; Criterion baselines
  cover `report` for multiple outputs and a test beside `layout_allocation.rs`
  counts its allocations; the 0.5.0 record cites the repair.
- Blocker: release authority and the owner-led provider/consumer sequence.
  Re-open when the owner starts that sequence; the repair landed in PR 126.
- Scope: `docs/adr/`, `crates/tyche-core/benches/`, workspace version.
- Basis: `ddde0f81`; priority: verification; needs: none

<a id="tyche-008"></a>

## TYCHE-008 — crates.io release automation [feature] — todo

- **Milestone:** M-1.0 (publication-boundary slice delivered; registry configuration and exact release evidence remain).
- Completed slice: make the publication boundary executable
  by keeping `tyche-core` publishable and the Consus, Moirai, and facade
  adapters private. The release workflow and registry configuration remain in
  the parent item after this slice.
- Basis: `30ca9dc9`; priority: feature; needs: none
- Scope: `crates/{tyche,tyche-consus,tyche-moirai}/Cargo.toml`, the distribution
  section of `README.md`, and this PM record.
- Acceptance: standalone locked package validation and focused tests pass; the
  crate is indexed; the exact-source GitHub Release exists; crates.io accepts
  only trusted-publisher updates.
- Evidence for this slice: package manifests set `publish = false` on all
  three integration packages, `publish = true` remains explicit on
  `tyche-core`, and README/CHANGELOG/checklist now state the same boundary.
  `cargo +1.97.0 fmt --all -- --check` and locked no-dependency metadata pass.
- Current clean-lane verification at `origin/main` commit `89194f3`: full
  workspace check and warning-denied Clippy pass; `cargo nextest run` passes
  51/51; doctests pass (18 total); Rustdoc, the reproducible-study example,
  benchmark smoke, and `cargo publish --dry-run --package tyche-core` pass.
  `cargo deny` is not installed in this environment, so the workflow's
  supply-chain job remains the authority for that gate.
- The direct local `mdbook test` invocation cannot be treated as a provider
  failure under the shared Atlas target: the target contains historical
  `tyche_core` artifacts and rustdoc reports multiple candidates. The caller
  already builds `tyche-core` and stages a fresh dependency directory before
  testing; hosted Pages evidence remains required. No GitHub Release or
  registry publication was performed because that is an explicit release
  state outside this implementation slice.

<a id="tyche-004"></a>

## TYCHE-004 — UQ breadth [feature] — todo

- **Milestone:** M-uq-closure (delivered scope stays closed; the remaining trainable-model seam for ensemble bagging joins M-uq-closure).
- Basis: `30ca9dc9`; priority: feature; needs: none
- Sensitivity-estimator source `dc96f5ecd6af643e34f2146b9f3dbb49ba85dbae`
  merged through PR #18 as provider default `4a6f8cd495c78beaaa6e4081705b33ed0da8be9e`.
  Provider verification and supply-chain checks passed in hosted run
  `31675476210`; book build passed in `31675476477`.

- First increment delivered in `tyche-core`: `Bootstrap` validates runtime
  population and resample sizes, exposes deterministic random-access indices,
  fills caller-owned storage without allocation, and reuses the canonical
  multiply-high bounded-integer kernel under a dedicated bootstrap domain.
  Provider-owned index generation is now complete; percentile interpolation
  remains a consumer policy.
- Consumer closure: Kwavers' elastography percentile bootstrap now delegates
  deterministic index generation to `Bootstrap::<SplitMix64>` in
  `tyche-core`; percentile interpolation remains Kwavers-owned. The consumer
  source slice is `crates/kwavers-analysis/src/signal_processing/
  estimation_bounds.rs`, with locked metadata and source formatting verified.
  This is a replay-boundary migration: consumers persisting results must record
  `SplitMix64::VERSION` and must not compare new output vectors with the former
  local continuous modulo schedule as if they were identical.
  The nominal ensemble bagging path remains separate and is not a valid
  migration target until it owns a real trainable-model seam: the current
  model stores only an error-derived score and prediction perturbs an external
  predictor without retained training.
- Delivered in the sensitivity-estimator increment: genuine Morris elementary
  effects and Saltelli first/total-order Sobol estimators, merged through PR #9
  as `deabe0b`; the current default CI run `31645405288` passes the full
  provider gate.
- Multi-output correlation, Morris, and Saltelli reports are delivered in the
  current increment through an `OUTPUTS` const-generic dimension. Scalar APIs
  remain the default specialization; two-output analytical laws pass in the
  `tyche-core` package suite.
- Remaining scope: a real trainable-model seam for ensemble bagging. The
  current score-only model is not represented as a trainable implementation.

<a id="tyche-005"></a>

## TYCHE-005 — Study schema [feature] — todo

- **Milestone:** M-study-schema (durability waits on Consus transaction support as a dependency, not ladder content).
- Basis: `30ca9dc9`; priority: feature; needs: none
- Versioned metadata/payload schema and manifest-last completeness; durability
  waits for Consus transaction support.
