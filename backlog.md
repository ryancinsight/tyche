# Tyche implementation backlog

<a id="tyche-correlation-range-002"></a>
## TYCHE-CORRELATION-RANGE-002 — Correlation range release evidence [major] — blocked
- priority: verification; milestone: M-uq-closure (planned 0.5.0, published); needs: none; basis: `0663169b4ab801812b1c6a086a06cd64b8f4ed59`.
- Outcome: ADR, midpoint, `report` benchmark and allocation evidence for the correlation-range repair merged as `ddde0f81`, and a 0.5.0 record that cites it.
- Acceptance: ADR 0007 (0006 is the 1.0 release scope) is Accepted, indexed and records the provider hold; a midpoint-index test passes; Criterion baselines cover `report` for multiple outputs; a test beside `layout_allocation.rs` counts `report` allocations; `CHANGELOG.md` lists the repair under 0.5.0.
- Evidence at the basis: `docs/adr/README.md` lists no 0007; `correlation_range.rs` has no midpoint case; `benches/counter_sampling.rs` has no `report` group; `layout_allocation.rs` never calls `report`; `CHANGELOG.md` files the repair under Unreleased although release commit `c49c984f` contains `ddde0f81`.
- Scope: `docs/adr/`, `crates/tyche-core/benches/`, `crates/tyche-core/tests/main/`, `CHANGELOG.md`.
- Blocker: the hold recorded on the claim head `refs/pull/117/head`: ADR 0006 stays Proposed until the owner-led provider/consumer sequence is qualified. ADR 0006 is Accepted and 0.5.0 is published (`gap_audit.md`, Tyche Core 0.5.0 release verification); no tracked file records whether that sequence is qualified.
- Question: is the owner-led provider/consumer sequence qualified? Method: the owner answers in the batched Ask-User round and the answer lands as a line in `gap_audit.md`. Re-open trigger: that line; on yes the status becomes todo.
- Next step: draft ADR 0007 and its index row from the `correlation_range.rs` cases, then add the midpoint, `report` benchmark and allocation cases.

<a id="tyche-stable-qualification"></a>
## TYCHE-STABLE-QUALIFICATION — tyche stable-release qualification [major] — todo
- priority: verification; milestone: M-uq-closure (a non-feature item takes the current milestone; its entries run on through M-study-schema and M-1.0); basis: `f490b7bb3bae`.
- Outcome: Close the durable study contract after the 0.5.0 Core release. Follow the [release scope](docs/adr/0006-release-scope-1-0.md) and [milestone exits](backlog/milestones.md). Planning targets for `tyche-core` are 0.5.1 only for compatible repairs, then 0.6.0, then 1.0.0. Verification alone can advance the current version.
- Acceptance and sequence: the entries below, in order. Representative manifest `tyche-core 0.5.0`; target 1.0.0.
- [ ] [patch] Reconcile the released Core receipt with TYCHE-CORRELATION-RANGE-002. Check the status of the proposed correlation repair independently and replay the deterministic stream and UQ value oracles.
- [ ] [minor] Exercise the versioned metadata and payload schema through the real Consus adapter. Recover a complete study by manifest and reject missing, reordered or corrupted payloads as its contract requires.
- [ ] [major] Verify the Core registry endpoint and private adapter boundaries, owned RNG reference cases and the clean study consumer before the stable API freeze.
- [ ] [patch] **0.5.1 only for compatible repairs.** Replay the correlation-range case at both endpoints and trace the candidate’s random stream from its recorded seed. Review any changed rejection contract against the board’s major classification. [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [minor] **0.6.0.** Persist a complete study with the Consus adapter and recover it by manifest. Compare metadata, array shape, payload bytes and sampled values, then check missing or corrupted payload handling.
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. An independent study reader reproduces the recorded results and rejects an inconsistent archive. Keep private adapters on their declared channel. Repeat the applicable exits on the exact RC before stable promotion. Run the [release scope](docs/adr/0006-release-scope-1-0.md) exits and attach reviewed, exact-revision evidence for this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).

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
- priority: feature; needs: none; basis: `0663169b4ab801812b1c6a086a06cd64b8f4ed59`.
- Outcome: a study persists through `tyche-consus` under a versioned metadata and payload schema with the manifest written last, so a reader recovers a complete study or rejects an incomplete one. [ADR 0001](docs/adr/0001-reproducible-study-boundary.md) (Persistence boundary) claims logical completeness only, not crash durability.
- Acceptance: the M-study-schema exit check. A test persists a complete study, recovers it by manifest and compares metadata, array shape, payload bytes and sampled values, and rejects missing, reordered and corrupted payloads with typed errors. At the basis `crates/tyche-consus/tests/archive_roundtrip.rs` holds one byte round-trip and key-safety test, and `git grep -n -i manifest -- crates` is empty.
- Scope: `crates/tyche-consus/src/`, `crates/tyche-consus/tests/`, `docs/adr/`.
- Question: does Consus expose a flush or transaction contract yet? ADR 0001 records none. Method: read the Consus `Store` trait at the locked revision; a contract adds a durability item and leaves this one unchanged.
- Next step: record the schema version header and the manifest-last write order in an ADR beside ADR 0001, then add the schema with the round-trip test.
