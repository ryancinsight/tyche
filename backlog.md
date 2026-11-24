# Tyche implementation backlog

<a id="tyche-stable-qualification"></a>
## TYCHE-STABLE-QUALIFICATION — tyche stable-release qualification [patch] — todo
- priority: verification; milestone: M-study-schema; basis: `f490b7bb3bae`.
- Outcome: Close the durable study contract after the 0.5.0 Core release. Follow the [release scope](docs/adr/0006-release-scope-1-0.md) and [milestone exits](backlog/milestones.md). Planning targets for `tyche-core` are 0.5.1 only for compatible repairs, then 0.6.0, then 1.0.0. Verification alone can advance the current version.
- Acceptance and sequence: the entries below, in order. Representative manifest `tyche-core 0.5.0`; target 1.0.0.
- [ ] [patch] Reconcile the released Core receipt with TYCHE-CORRELATION-RANGE-002. Check the status of the proposed correlation repair independently and replay the deterministic stream and UQ value oracles.
- [ ] [minor] Exercise the versioned metadata and payload schema through the real Consus adapter. Recover a complete study by manifest and reject missing, reordered or corrupted payloads as its contract requires.
- [ ] [major] Verify the Core registry endpoint and private adapter boundaries, owned RNG reference cases and the clean study consumer before the stable API freeze.
- [ ] [patch] **0.5.1 only for compatible repairs.** Replay the correlation-range case at both endpoints and trace the candidate’s random stream from its recorded seed. Review any changed rejection contract against the board’s major classification. [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [minor] **0.6.0.** Persist a complete study with the Consus adapter and recover it by manifest. Compare metadata, array shape, payload bytes and sampled values, then check missing or corrupted payload handling.
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. An independent study reader reproduces the recorded results and rejects an inconsistent archive. Keep private adapters on their declared channel. Repeat the applicable exits on the exact RC before stable promotion. Run the [release scope](docs/adr/0006-release-scope-1-0.md) exits and attach reviewed, exact-revision evidence for this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).

<a id="tyche-correlation-range-002"></a>

## TYCHE-CORRELATION-RANGE-002 — Correlation range release evidence [major] — blocked

- **Milestone:** M-uq-closure (planned version 0.5.0, the release this item records).
- Outcome: 0.5.0 bump, correlation-repair ADR, provider hold, midpoint tests and
  `report` evidence that the repair dropped from released claim `refs/pull/117/head`.
- Acceptance: ADR 0007 (0006 is the 1.0 release scope) is Accepted, indexed and
  records the provider hold; a midpoint-index test passes; Criterion baselines
  cover `report` for multiple outputs and a test beside `layout_allocation.rs`
  counts its allocations; the 0.5.0 record cites the repair.
- The 0.5.0 provider release is complete. The correlation-specific ADR,
  benchmark, and allocation evidence remain open for the repair in PR 126.
- Scope: `docs/adr/`, `crates/tyche-core/benches/`, workspace version.
- Basis: `ddde0f81`; priority: verification; needs: none

<a id="tyche-004"></a>

## TYCHE-004 — Trainable-model seam for ensemble bagging [minor] — todo

- **Milestone:** M-uq-closure (the delivered bootstrap, Morris, Saltelli and multi-output reports stay closed; only the seam remains).
- Outcome: `tyche-core` owns a trainable-model contract that retains trained state, so consumer ensemble bagging can adopt provider resampling instead of a score-only model whose prediction perturbs an external predictor.
- Evidence at `0663169b4ab8`: `crates/tyche-core/src/ensemble/model.rs:4-22` holds only the evaluation contract `StudyModel::evaluate`; `git grep -n -i -e trainable -e bagging -- crates` is empty; [ADR 0006](docs/adr/0006-release-scope-1-0.md) lines 15 and 24-25 and `README.md:197-200` name the seam as open.
- Acceptance: the M-uq-closure exit check. `cargo test -p tyche-core --locked --all-features` runs an ensemble-bagging case over a retained trainable model; no such test exists today.
- Replay boundary: consumers persisting bootstrap results record `SplitMix64::VERSION` and never compare them with a former local modulo schedule.
- priority: feature; needs: none
- Scope: `crates/tyche-core/src/ensemble/`, `crates/tyche-core/tests/main/`, `docs/adr/`.
- Next step: record the retained-state training contract in an ADR beside `StudyModel`, then add it with the bagging test.
- Basis: `0663169b4ab801812b1c6a086a06cd64b8f4ed59`

<a id="tyche-005"></a>

## TYCHE-005 — Study schema [minor] — todo

- **Milestone:** M-study-schema (durability waits on Consus transaction support as a dependency, not ladder content).
- Basis: `30ca9dc9`; priority: feature; needs: none
- Versioned metadata/payload schema and manifest-last completeness; durability
  waits for Consus transaction support.

<a id="tyche-008"></a>

## TYCHE-008 — crates.io release automation [patch] — todo

- **Milestone:** M-1.0 (the 0.5.0 registry publication is delivered; the independent-consumer replay remains).
- Outcome: `tyche-core` publishes to crates.io only through Trusted Publishing from an exact-source GitHub Release, and the Consus, Moirai and facade adapters stay private with `publish = false`.
- Evidence at `0663169b4ab8`: crates.io lists `tyche-core` 0.5.0 with `trustpub_only` true (`https://crates.io/api/v1/crates/tyche-core`); release `tyche-core-v0.5.0` targets `c49c984f5fbe`, an ancestor of default; `Crates.io Release` run `44782145848` concluded success on that commit; `gap_audit.md` section "Tyche Core 0.5.0 release verification" records the archive hash match and the replay.
- Acceptance: an independent consumer that generates its lock from scratch resolves 0.5.0 from crates.io and reproduces the four version-one counter words, and the evidence facts above hold.
- priority: feature; needs: none
- Scope: `crates/tyche-core/Cargo.toml`, `.github/workflows/rust-release.yml`, `gap_audit.md`.
- Next step: rerun that replay against the registry artifact, then delete this item in the commit that carries `Item: TYCHE-008`.
- Basis: `0663169b4ab801812b1c6a086a06cd64b8f4ed59`
