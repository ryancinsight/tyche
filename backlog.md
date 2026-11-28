# Tyche implementation backlog

<a id="tyche-correlation-range-002"></a>
## TYCHE-CORRELATION-RANGE-002 — Correlation range evidence [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `7ca7f7276ff4dbafb59e1124cf167e0a2a2c092c`.
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
- priority: verification; milestone: M-uq-closure (a non-feature item takes the current milestone; its entries run on through M-study-schema and M-1.0); basis: `fcfd3d90e2a45eb0093d4cde37125beed88e1b5d`.
- Outcome: Close the durable study contract after the 0.5.0 Core release. Follow the [release scope](docs/adr/0006-release-scope-1-0.md) and [milestone exits](backlog/milestones.md). Planning targets for `tyche-core` are 0.5.1 only for compatible repairs, cut from a branch at `tyche-core-v0.5.0` because `main` holds the unreleased [minor] seam (`CHANGELOG.md` `## Unreleased`), then 0.6.0 (M-uq-closure), 0.7.0 (M-study-schema), then 1.0.0. Verification alone can advance the current version.
- Acceptance and sequence: the entries below, in order. Representative manifest `tyche-core 0.5.0`; target 1.0.0.
- [ ] [patch] Depends on TYCHE-CORRELATION-RANGE-002; a `needs:` line would park the other entries. Reconcile the released Core receipt with it: confirm independently that the correlation repair merged as `ddde0f81` is in the released source and replay the deterministic stream and UQ value oracles.
- [ ] [minor] Exercise the versioned metadata and payload schema through the real Consus adapter. Recover a complete study by manifest and reject missing, reordered or corrupted payloads as its contract requires.
- [ ] [major] Verify the Core registry endpoint and private adapter boundaries, owned RNG reference cases and the clean study consumer before the stable API freeze.
- [ ] [patch] **0.5.1 only for compatible repairs, cut from a branch at `tyche-core-v0.5.0`.** Replay the correlation-range case at both endpoints and trace the candidate’s random stream from its recorded seed. Review any changed rejection contract against the board’s major classification. [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [minor] **0.7.0.** Persist a complete study with the Consus adapter and recover it by manifest. Compare metadata, array shape, payload bytes and sampled values, then check missing or corrupted payload handling.
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. An independent study reader reproduces the recorded results and rejects an inconsistent archive. Keep private adapters on their declared channel. Repeat the applicable exits on the exact RC before stable promotion. Run the [release scope](docs/adr/0006-release-scope-1-0.md) exits and attach reviewed, exact-revision evidence for this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).
- Scope: `gap_audit.md`, `CHANGELOG.md`, `docs/adr/`, `crates/tyche-core/tests/main/stream_vectors.rs`, `crates/tyche-consus/`, `crates/tyche/examples/reproducible_study.rs`, `.github/workflows/rust-release.yml`.
- Next step: the registry-endpoint and private-adapter checks of the third entry run now against the `publish` keys and `rust-release.yml`; entry 1 follows TYCHE-CORRELATION-RANGE-002; entries 2 and 5 exercise the schema that TYCHE-005 adds.

<a id="tyche-adr-0007-accept"></a>
## TYCHE-ADR-0007-ACCEPT — Accept the correlation release-boundary ADR [patch] — blocked
- priority: verification; milestone: M-uq-closure; needs: TYCHE-CORRELATION-RANGE-002; basis: `7ca7f7276ff4dbafb59e1124cf167e0a2a2c092c`.
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
- priority: feature; needs: none; basis: `7ca7f7276ff4dbafb59e1124cf167e0a2a2c092c`.
- Outcome: a study persists through `tyche-consus` under a versioned metadata and payload schema with the manifest written last, so a reader recovers a complete study or rejects an incomplete one. [ADR 0001](docs/adr/0001-reproducible-study-boundary.md) (Persistence boundary) claims logical completeness only, not crash durability.
- Acceptance: the M-study-schema exit check. A test persists a complete study, recovers it by manifest and compares metadata, array shape, payload bytes and sampled values, and rejects a payload with an unsupported schema version and missing, reordered and corrupted payloads with typed errors. At the basis `crates/tyche-consus/tests/archive_roundtrip.rs` holds one byte round-trip and key-safety test, and `git grep -n -i manifest -- crates` is empty.
- Scope: `crates/tyche-consus/src/`, `crates/tyche-consus/tests/`, `docs/adr/`.
- Question: does Consus expose a flush or transaction contract yet? ADR 0001 records none. Method: read the Consus `Store` trait at the locked revision; a contract adds a durability item and leaves this one unchanged.
- Next step: record the schema version header and the manifest-last write order in an ADR beside ADR 0001, then add the schema with the round-trip test.

<a id="tyche-trainable-001"></a>
## TYCHE-TRAINABLE-001 — Decide ADR 0009's open consumer questions [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `7ca7f7276ff4dbafb59e1124cf167e0a2a2c092c`.
- Outcome: ADR 0009 records three decisions it omits. (1) Multi-output: `TrainableModel<T>` returns one `T` per prediction, and the Kwavers consumer (`ensemble_methods/mod.rs`, `Array2<f32>` predictions, per-element variance) predicts arrays. (2) Member spread: `BaggedModel::predict` returns only the mean (`bagging.rs:115-123`), while the consumer reports variance. (3) Bound: `fitted_members` and `predict` need `T: RealField` (`bagging.rs:99`) but `Bagging::fit` has no such bound, so a non-scalar `T` fits and cannot be read.
- Acceptance: ADR 0009 states for each of the three the chosen option, the strongest rejected one and the overturning evidence; with the tyche checkout root as working directory and the scripts taken from an export of the stack repository's default branch at its fetched revision, `python <export>/scripts/adr-index.py --strict --directory docs/adr check` and `python <export>/scripts/atlas-adr-canonical-form.py --check docs/adr` pass (tyche has no `scripts/adr-index.py`; both scripts live in the stack repository's `scripts/`); any chosen API change is filed as its own item citing the ADR.
- Verification at the basis: read from `bagging.rs` and from `repos/kwavers` at `origin/main` `7e9ac5d9`; `git grep -n -i -E 'spread|multi-output|RealField' docs/adr/0009-retained-state-trainable-models.md` is empty.
- Scope: `docs/adr/0009-retained-state-trainable-models.md`.
- Next step: draft the three decisions as the recommended option and name them veto points.

<a id="tyche-ladder-001"></a>
## TYCHE-LADDER-001 — Rewrite the milestone ladder to the roadmap rules [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `7ca7f7276ff4dbafb59e1124cf167e0a2a2c092c`.
- Outcome: `backlog/milestones.md` has three defects against the roadmap rules. M-study-schema's exit (`milestones.md:6`) is the whole-workspace `cargo test --workspace --locked --all-features`, which a hosted run at `1f74f96` passes with no schema (60 of 60 tests; judge-cited run 45338630845, not rerun here). M-1.0 (`:8-9`) has no `exit:` line. No milestone has an example exit check that checks its own result.
- Acceptance: every milestone has an `exit:` of one `<action> <result> <object>` feature with a named runnable check that fails without its feature and, for each milestone still open at the rewrite commit, fails there; one per milestone a runnable example that checks its own result. M-uq-closure: its exit test `bagging::bagged_prediction_is_the_member_mean_and_reuses_fitted_state_for_every_scalar` exists (`crates/tyche-core/tests/main/bagging.rs:148`) and is reported passing, so only a commit that regresses it fails it, and the fails-there clause does not apply to this block. If the block is still open when the rewrite starts, the closing sweep decides it first: its standing conditions pass and the sweep deletes it, or they fail and the rewrite leaves the block and its `standing:` unchanged. BAGGING-001 was the only open `correctness` item (`git grep -n '^- priority: correctness' -- backlog.md` printed only its line until its delivering PR deleted the block), M-study-schema's exit is the named TYCHE-005 test; TYCHE-005 names none and the tree holds none (`git grep -n -i manifest -- crates` is empty), so the rewrite names it and TYCHE-005's delivering PR adds it under that name. The first milestone's `standing:` keeps the installed-state condition TYCHE-LADDER-002 added. A revision note on ADR 0006 names each moved check. A judge verdict confirms every capability area and open feature item maps to an exit check, a non-goal or delivered capability, and each open milestone has one check failing at that commit.
- Scope: `backlog/milestones.md`, `docs/adr/0006-release-scope-1-0.md`.
- Next step: draft the M-study-schema and M-1.0 exit checks from TYCHE-005's acceptance and `crates/tyche/examples/reproducible_study.rs`, then open the rewrite under `judge-required`.

<a id="tyche-docs-001"></a>
## TYCHE-DOCS-001 — Facade registry claims disagree with the registry [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `1f74f9677a3ba2382bc11df029ea610ee59b27ed`.
- Outcome: README and the facade manifest stop claiming a 0.4 registry release. `README.md:32-34` says the facade is `publish = false`; `README.md:38-43` and `crates/tyche/Cargo.toml:3-8` tell consumers to depend on `tyche-uncertainty` at `version = "0.4"` from the registry. crates.io lists `tyche-uncertainty` at 0.2.0 only (looked up 2026-11-24, `max_version` 0.2.0) while the workspace version is 0.4.0 and the facade is `publish = false`.
- Acceptance: `git grep -n 'version = "0.4"' -- README.md crates/tyche/Cargo.toml` is empty, README states the Git dependency a consumer of the current facade uses, and the README example compiles in a clean consumer crate outside the workspace that depends on that source: `cargo generate-lockfile`, then `cargo check --locked` there (`--locked` alone fails in a fresh crate, which has no lock). Hypothesis, not run: a `version = "0.2"` edit fails the last clause.
- Choice: the Git dependency is the recommended option (reversible, within Change authority). The alternatives are a facade release under release authority (publishing is authority-gated) or removing the facade from consumer guidance.
- Scope: `README.md`, `crates/tyche/Cargo.toml`.
- Next step: state the Git-dependency form in README and the manifest comment, then resolve it from a scratch consumer.

<a id="tyche-docs-002"></a>
## TYCHE-DOCS-002 — Book chapter on bootstrap and bagging [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `7ca7f7276ff4dbafb59e1124cf167e0a2a2c092c`.
- Outcome: `docs/book/SUMMARY.md` lists no bootstrap or bagging page (`git grep -n -i -E 'bootstrap|bagging' -- docs/book` finds only `stack_position.md`). Add one chapter with a runnable example whose bagged mean is checked against an independent sum.
- Outline: resampling with replacement and why the mean aggregation applies; the exact multiply-high reducer and the `(seed, replicate, draw)` address with replay under `SplitMix64::VERSION`; `Bootstrap::at` and allocation-free `fill_into`; the `TrainableModel` seam, `Fitted` ownership and the fit count; error variants; what the provider does not own (percentile policy).
- Acceptance: the commands book CI runs pass: `cargo build --locked -p tyche-core`, then `mdbook test docs/book --library-path <cargo target dir>/mdbook-test-libs` (`.github/workflows/book-pages.yml:38-47` calls the pinned Atlas reusable workflow `book-pages.yml` with mdBook 0.5.4, toolchain 1.97.0). That library directory holds `tyche-core` and its dependencies, not the facade, so the example is `crates/tyche-core/examples/book_bagging.rs` importing `tyche_core`, included as `docs/book/examples/moments.md:14-16` includes `book_moments.rs`, in a fenced block that is not `ignore` (an `ignore` block passes without compiling); `SUMMARY.md` lists the chapter.
- Scope: `docs/book/`, `crates/tyche-core/examples/`.
- Next step: write the derivation first, then the example, then wire the include.

<a id="tyche-ci-aggregate-001"></a>
## TYCHE-CI-AGGREGATE-001 — One aggregate check that refuses drafts [patch] — todo
- priority: verification; milestone: M-uq-closure; needs: none; basis: `7ca7f7276ff4dbafb59e1124cf167e0a2a2c092c`.
- Outcome: `.github/workflows/ci.yml` has jobs `semver`, `lockfile`, `verify` and `supply-chain` but no aggregate, and `on.pull_request` sets no `types`. The required contexts are job names: `gh api repos/ryancinsight/tyche/branches/main/protection/required_status_checks --jq .contexts` returned `["verify","supply-chain","Lockfile integrity / Lockfile integrity"]` (2026-11-24). The stack's workflow rule requires one named aggregate that `needs:` every job and fails on any failure, cancellation or draft, so a draft never reads as passing (judge finding F8 on tyche#144: `verify` passed on a draft).
- Acceptance: `git grep -n -E 'if: (\$\{\{ )?always\(\)' -- .github/workflows/ci.yml` prints the aggregate job's line (no match at the basis); `on.pull_request.types` in that file lists `ready_for_review`; once a green run names the job, the `gh api` command above returns that one context; on a draft pull request the aggregate job concludes `failure`.
- Scope: `.github/workflows/ci.yml`; the repository's branch protection (merge-mechanics setting).
- Next step: add the aggregate (`semver` is informational on pull requests, so its result is not an aggregate failure) and `types`, run it on a draft and a ready pull request, then replace the required contexts through `gh api`.
