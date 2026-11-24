# Tyche implementation backlog

<a id="tyche-correlation-range-002"></a>
## TYCHE-CORRELATION-RANGE-002 — Correlation range evidence [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `0663169b4ab801812b1c6a086a06cd64b8f4ed59`.
- Outcome: draft ADR, rounding-midpoint tests, `report` benchmark, allocation test and changelog placement for the repair merged as `ddde0f81`, which release commit `c49c984f` (0.5.0) contains. None adds a public contract, so the class is [patch]. The Accepted flip is TYCHE-ADR-0007-ACCEPT; nothing here waits on it.
- ADR: `docs/adr/0007-correlation-provider-release-boundary.md`, `Status: Proposed`, with its index row. Source text: `docs/adr/0006-correlation-provider-release-boundary.md` at `refs/pull/117/head`, rewritten to the as-built facts (0.5.0 published at `c49c984f`, Eunomia 0.12 per ADR 0008), plus the numerical contract of `docs/correlation-range.md` there (absent on main).
- Midpoint test: restore `crates/tyche-core/src/statistics/sensitivity/correlation_rounding_tests.rs` from `refs/pull/117/head`, mounted by `#[path]` in `correlation.rs`. "Midpoint" is the rounding midpoint at the least subnormal unit (`docs/correlation-range.md:31-34` there): the binary32 and binary64 witnesses lie within one epsilon of 1/2 or 3/2 units and must satisfy `|actual_units - units| <= 0.5 + 8*EPSILON`, and zero versus nonzero stays strict away from the midpoint, for f32, f64, F32 and F64.
- Benchmark: a Criterion group over `CorrelationScreening::report` at one and several outputs under `crates/tyche-core/benches/`, whose timed closure calls `report` and returns its result through `black_box`, with a stored baseline and a passing single-iteration smoke.
- Allocation: restore `construction_updates_and_report_allocate_nothing` (`crates/tyche-core/tests/correlation_range_boundaries.rs:113` at `refs/pull/117/head`) beside `layout_allocation.rs`; it asserts `allocations == AllocationDelta::default()`, a zero count by value, around construction, updates and `report` for f32, F32, f64 and F64.
- Changelog: move the `[patch]` correlation entry from `## Unreleased` to a `### Fixed` section under `## 0.5.0` in `CHANGELOG.md`; `## Unreleased` then holds no correlation entry and the entry occurs once.
- Evidence at the basis: `docs/adr/README.md` lists no 0007; the midpoint witnesses and the correlation contract exist only at `refs/pull/117/head`; `benches/counter_sampling.rs` has no `report` group; `layout_allocation.rs` never calls `report`; the repair sits under Unreleased.
- Scope: `docs/adr/`, `crates/tyche-core/src/statistics/sensitivity/`, `crates/tyche-core/benches/`, `crates/tyche-core/tests/main/`, `CHANGELOG.md`.
- Next step: restore the allocation and midpoint tests first (they pin existing behavior), then the benchmark, the changelog move and the ADR.

<a id="tyche-stable-qualification"></a>
## TYCHE-STABLE-QUALIFICATION — tyche stable-release qualification [major] — todo
- priority: verification; milestone: M-uq-closure (a non-feature item takes the current milestone; its entries run on through M-study-schema and M-1.0); basis: `f490b7bb3bae`.
- Outcome: Close the durable study contract after the 0.5.0 Core release. Follow the [release scope](docs/adr/0006-release-scope-1-0.md) and [milestone exits](backlog/milestones.md). Planning targets for `tyche-core` are 0.5.1 only for compatible repairs before the seam release, then 0.6.0 (M-uq-closure), 0.7.0 (M-study-schema), then 1.0.0. Verification alone can advance the current version.
- Acceptance and sequence: the entries below, in order. Representative manifest `tyche-core 0.5.0`; target 1.0.0.
- [ ] [patch] Depends on TYCHE-CORRELATION-RANGE-002; a `needs:` line would park the other entries. Reconcile the released Core receipt with it: confirm independently that the correlation repair merged as `ddde0f81` is in the released source and replay the deterministic stream and UQ value oracles.
- [ ] [minor] Exercise the versioned metadata and payload schema through the real Consus adapter. Recover a complete study by manifest and reject missing, reordered or corrupted payloads as its contract requires.
- [ ] [major] Verify the Core registry endpoint and private adapter boundaries, owned RNG reference cases and the clean study consumer before the stable API freeze.
- [ ] [patch] **0.5.1 only for compatible repairs.** Replay the correlation-range case at both endpoints and trace the candidate’s random stream from its recorded seed. Review any changed rejection contract against the board’s major classification. [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [minor] **0.7.0.** Persist a complete study with the Consus adapter and recover it by manifest. Compare metadata, array shape, payload bytes and sampled values, then check missing or corrupted payload handling.
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. An independent study reader reproduces the recorded results and rejects an inconsistent archive. Keep private adapters on their declared channel. Repeat the applicable exits on the exact RC before stable promotion. Run the [release scope](docs/adr/0006-release-scope-1-0.md) exits and attach reviewed, exact-revision evidence for this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).
- Scope: `gap_audit.md`, `CHANGELOG.md`, `docs/adr/`, `crates/tyche-core/tests/main/stream_vectors.rs`, `crates/tyche-consus/`, `crates/tyche/examples/reproducible_study.rs`, `.github/workflows/rust-release.yml`.
- Next step: the registry-endpoint and private-adapter checks of the third entry run now against the `publish` keys and `rust-release.yml`; entry 1 follows TYCHE-CORRELATION-RANGE-002; entries 2 and 5 exercise the schema that TYCHE-005 adds.

<a id="tyche-adr-0007-accept"></a>
## TYCHE-ADR-0007-ACCEPT — Accept the correlation release-boundary ADR [patch] — blocked
- priority: verification; milestone: M-uq-closure; needs: TYCHE-CORRELATION-RANGE-002; basis: `0663169b4ab801812b1c6a086a06cd64b8f4ed59`.
- Outcome: ADR 0007 moves from Proposed to Accepted and its index row follows.
- Blocker: the hold recorded in `backlog.md` at `refs/pull/117/head`: "ADR 0006 remains Proposed until the owner-led provider/consumer sequence is qualified". There "ADR 0006" is the correlation ADR now numbered 0007; on main ADR 0006 is the 1.0 release scope, a different document.
- Question to the owner: is the owner-led provider/consumer sequence qualified, so ADR 0007 may be Accepted?
- Assumption meanwhile: ADR 0007 stays Proposed; the work in TYCHE-CORRELATION-RANGE-002 proceeds and waits on nothing here.
- Re-open trigger: the owner's answer recorded in `gap_audit.md`. On yes, set `Status: Accepted` and regenerate the index; on no, record the unmet step there and keep the ADR Proposed.
- Acceptance: ADR 0007 reads `Status: Accepted` and `docs/adr/README.md` lists it Accepted; `python scripts/adr-index.py check --directory docs/adr` passes from an Atlas checkout.
- Scope: `docs/adr/0007-correlation-provider-release-boundary.md`, `docs/adr/README.md`.

<a id="tyche-005"></a>
## TYCHE-005 — Study schema [minor] — todo
- **Milestone:** M-study-schema (durability waits on Consus transaction support as a dependency, not ladder content).
- priority: feature; needs: none; basis: `0663169b4ab801812b1c6a086a06cd64b8f4ed59`.
- Outcome: a study persists through `tyche-consus` under a versioned metadata and payload schema with the manifest written last, so a reader recovers a complete study or rejects an incomplete one. [ADR 0001](docs/adr/0001-reproducible-study-boundary.md) (Persistence boundary) claims logical completeness only, not crash durability.
- Acceptance: the M-study-schema exit check. A test persists a complete study, recovers it by manifest and compares metadata, array shape, payload bytes and sampled values, and rejects a payload with an unsupported schema version and missing, reordered and corrupted payloads with typed errors. At the basis `crates/tyche-consus/tests/archive_roundtrip.rs` holds one byte round-trip and key-safety test, and `git grep -n -i manifest -- crates` is empty.
- Scope: `crates/tyche-consus/src/`, `crates/tyche-consus/tests/`, `docs/adr/`.
- Question: does Consus expose a flush or transaction contract yet? ADR 0001 records none. Method: read the Consus `Store` trait at the locked revision; a contract adds a durability item and leaves this one unchanged.
- Next step: record the schema version header and the manifest-last write order in an ADR beside ADR 0001, then add the schema with the round-trip test.

<a id="tyche-bagging-001"></a>
## TYCHE-BAGGING-001 — Bagging resample allocation panics [minor] — todo
- priority: correctness; milestone: M-uq-closure; needs: none; basis: `1f74f9677a3ba2382bc11df029ea610ee59b27ed`.
- Outcome: `Bagging::fit` returns a typed error where it now panics. `Bootstrap::new(1, usize::MAX)` is accepted (`crates/tyche-core/src/sampling/resampling.rs:32-46` checks only `u64` addressability), then `vec![0; self.bootstrap.resample_size()]` (`crates/tyche-core/src/ensemble/bagging.rs:57`) overflows capacity. The `# Panics` text "Never in practice" (`bagging.rs:37-40`) is false for that input, and any resample too large to allocate reaches the allocator abort.
- Acceptance: test `bagging::fit_rejects_an_unallocatable_resample_with_a_typed_error` in `crates/tyche-core/tests/main/bagging.rs` fits `Bootstrap::<SplitMix64>::new(1, usize::MAX)` over one observation and matches a new `BaggingError` variant carrying the resample size; run `cargo nextest run -p tyche-core --locked --all-features --test main -E 'test(=bagging::fit_rejects_an_unallocatable_resample_with_a_typed_error)'`. It fails today with a capacity-overflow panic.
- Design: reserve with `Vec::try_reserve_exact`, then fill. `BaggingError` is `#[non_exhaustive]`, so the added variant is [minor]. The `# Panics` section then names only the `expect` whose invariant `fill_into` establishes.
- Verification at the basis: read from source (the cited lines); the panic itself is not run.
- Scope: `crates/tyche-core/src/ensemble/bagging.rs`, `crates/tyche-core/tests/main/bagging.rs`, `docs/adr/0009-retained-state-trainable-models.md` (error list).
- Next step: add the failing test, then the reservation and variant.

<a id="tyche-bagging-002"></a>
## TYCHE-BAGGING-002 — Bagging tests that kill surviving mutants [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `1f74f9677a3ba2382bc11df029ea610ee59b27ed`.
- Outcome: three tests in `crates/tyche-core/tests/main/bagging.rs` (its only error case is an empty slice and `OriginLine::predict` never fails), each failing under one mutant of `crates/tyche-core/src/ensemble/bagging.rs`:
  - `bagging::over_length_observations_report_population_length`: ten observations against population nine return `PopulationLength { expected: 9, actual: 10 }` and fit no member; kills `!=` to `<` at line 51.
  - `bagging::model_failure_exposes_its_source`: `BaggingError::Model(e).source()` is `Some` and is that error; `PopulationLength` yields `None`; kills `source()` returning `None`.
  - `bagging::prediction_failure_propagates_from_the_first_member`: a model whose `predict` errors makes `BaggedModel::predict` return that error; kills a swallowed error at line 103.
- Acceptance: all three pass at the head and fail under their mutant; `cargo mutants -p tyche-core --file crates/tyche-core/src/ensemble/bagging.rs` reports no survivor on those lines.
- Also: reword `crates/tyche-core/src/ensemble/model.rs:41-43` "so predicting never refits" to the ADR 0009 contract wording (an implementor whose `Fitted` stores the observations can refit; the contract excludes it).
- Verification at the basis: source read; the survivors are derived from the test file's contents, no mutation run.
- Scope: `crates/tyche-core/tests/main/bagging.rs`, `crates/tyche-core/src/ensemble/model.rs`.
- Next step: add the three tests, run the mutants command, then the Rustdoc wording.

<a id="tyche-trainable-001"></a>
## TYCHE-TRAINABLE-001 — Decide ADR 0009's open consumer questions [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `1f74f9677a3ba2382bc11df029ea610ee59b27ed`.
- Outcome: ADR 0009 records three decisions it omits. (1) Multi-output: `TrainableModel<T>` returns one `T` per prediction, and the Kwavers consumer (`ensemble_methods/mod.rs`, `Array2<f32>` predictions, per-element variance) predicts arrays. (2) Member spread: `BaggedModel::predict` returns only the mean (`bagging.rs:100-108`), while the consumer reports variance. (3) Bound: `fitted_members` and `predict` need `T: RealField` (`bagging.rs:84`) but `Bagging::fit` has no such bound, so a non-scalar `T` fits and cannot be read.
- Acceptance: ADR 0009 states for each of the three the chosen option, the strongest rejected one and the overturning evidence; `python scripts/adr-index.py check --strict --directory docs/adr` and `atlas-adr-canonical-form.py --check` pass; any chosen API change is filed as its own item citing the ADR.
- Verification at the basis: read from `bagging.rs` and from `repos/kwavers` at `origin/main` `7e9ac5d9`; `git grep -n -i -E 'spread|multi-output|RealField' docs/adr/0009-retained-state-trainable-models.md` is empty.
- Scope: `docs/adr/0009-retained-state-trainable-models.md`.
- Next step: draft the three decisions as the recommended option and name them veto points.

<a id="tyche-docs-001"></a>
## TYCHE-DOCS-001 — Facade registry claims disagree with the registry [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `1f74f9677a3ba2382bc11df029ea610ee59b27ed`.
- Outcome: README and the facade manifest stop claiming a 0.4 registry release. `README.md:32-34` says the facade is `publish = false`; `README.md:38-43` and `crates/tyche/Cargo.toml:3-8` tell consumers to depend on `tyche-uncertainty` at `version = "0.4"` from the registry. crates.io lists `tyche-uncertainty` at 0.2.0 only (looked up 2026-11-24, `max_version` 0.2.0) while the workspace version is 0.4.0 and the facade is `publish = false`.
- Acceptance: `git grep -n 'version = "0.4"' -- README.md crates/tyche/Cargo.toml` is empty, and README states which source a consumer of the current facade uses; a clean consumer resolves that source.
- Open choice: a Git dependency, a facade release under release authority, or removing the facade from consumer guidance. Publishing is authority-gated.
- Scope: `README.md`, `crates/tyche/Cargo.toml`.
- Next step: state the Git-dependency form in README and the manifest comment, then resolve it from a scratch consumer.

<a id="tyche-docs-002"></a>
## TYCHE-DOCS-002 — Book chapter on bootstrap and bagging [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `1f74f9677a3ba2382bc11df029ea610ee59b27ed`.
- Outcome: `docs/book/SUMMARY.md` lists no bootstrap or bagging page (`git grep -n -i -E 'bootstrap|bagging' -- docs/book` finds only `stack_position.md`). Add one chapter with a runnable example whose bagged mean is checked against an independent sum.
- Outline: resampling with replacement and why the mean aggregation applies; the exact multiply-high reducer and the `(seed, replicate, draw)` address with replay under `SplitMix64::VERSION`; `Bootstrap::at` and allocation-free `fill_into`; the `TrainableModel` seam, `Fitted` ownership and the fit count; error variants; what the provider does not own (percentile policy).
- Acceptance: `mdbook test docs/book` passes with the example included from a compiled `crates/tyche/examples/` file through `{{#rustdoc_include}}`, and `SUMMARY.md` lists the chapter.
- Scope: `docs/book/`, `crates/tyche/examples/`.
- Next step: write the derivation first, then the example, then wire the include.
