# Tyche checklist

## Stable release checklist

Basis `f490b7bb3bae`, representative manifest `tyche-core 0.5.0`. Target 1.0.0. [Backlog item](backlog.md#tyche-stable-qualification).

- [ ] [patch] Reconcile the released Core receipt with TYCHE-CORRELATION-RANGE-002. Check the status of the proposed correlation repair independently and replay the deterministic stream and UQ value oracles.
- [ ] [minor] Exercise the versioned metadata and payload schema through the real Consus adapter. Recover a complete study by manifest and reject missing, reordered or corrupted payloads as its contract requires.
- [ ] [major] Verify the Core registry endpoint and private adapter boundaries, owned RNG reference cases and the clean study consumer before the stable API freeze.
- [ ] [patch] **0.5.1 only for compatible repairs.** Replay the correlation-range case at both endpoints and trace the candidate’s random stream from its recorded seed. Review any changed rejection contract against the board’s major classification. [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [minor] **0.6.0.** Persist a complete study with the Consus adapter and recover it by manifest. Compare metadata, array shape, payload bytes and sampled values, then check missing or corrupted payload handling.
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. An independent study reader reproduces the recorded results and rejects an inconsistent archive. Keep private adapters on their declared channel. Repeat the applicable exits on the exact RC before stable promotion. Run the [release scope](docs/adr/0006-release-scope-1-0.md) exits and attach reviewed, exact-revision evidence for this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).

## Current release target tyche-core 0.5.0

- [x] [major] Verify the API and normalized package for [ADR 0008](docs/adr/0008-registry-provider-release.md).
- [x] [major] Publish and verify an independent registry consumer.

- [x] ATLAS-TYCHE-TYPE-SURFACE-001 [patch]: consolidate the checked index
      conversions shared by Latin-hypercube and Sobol designs, rename the
      bounded counter helper to its domain contract, and remove all five
      production type-suffixed function names. Verification at delivery:
      conformance is zero across all tracked debt classes; nextest 51/51,
      doctests 18/18, Clippy `-D warnings`, and workspace documentation pass.

- [x] ATLAS-TYCHE-SENSITIVITY-STRUCTURE [patch]: split the 672-line
      `statistics/sensitivity.rs` implementation into dedicated estimator
      modules, preserve the public API and tests, and reduce the oversized-file
      conformance count from 1 to 0.

- [x] TYCHE-001: Phase 0 core — reproducible design, study, ensemble,
      statistics, calibration, execution, and artifact-access vertical slice.
- [x] TYCHE-002: Public promotion — public `ryancinsight/tyche` origin and the
      Atlas gitlink registered.
- [x] TYCHE-003: Sampling breadth — domain-separated versioned stream vectors,
      fixed/runtime random-access Sobol (one const-generic kernel), and ADR 0005
      categorical/weighted/discrete importance sampling; 40/40 workspace suite
      and 18/18 doctests at delivery.
- [x] TYCHE-007: Provider source consolidation — Eunomia 0.7 through its
      canonical versioned Git source; locked metadata and full gate set pass.
- [x] TYCHE-004: deterministic random-access bootstrap index generation
      (`Bootstrap::<SplitMix64>`) with validated sizes, caller-owned
      fill, and the shared multiply-high reducer; Kwavers elastography
      percentile bootstrap delegates to it. Morris elementary effects,
      Saltelli Sobol indices, and multi-output sensitivity reports are also
      implemented and covered by the statistics evidence suite.
- [x] Re-release at 0.2.0 onto Eunomia 0.8 (`45354e6`), facade packaging and
      documentation (`ee3a52b`), and CI package-selector fixes.
- [x] Canonical provider gates at the audited revision (2026-08-12): strict
      all-target check with `-D warnings`, Clippy `-D warnings`
      (all-targets/all-features), Nextest 50/50 (all-features), 17/17
      doctests, and the `--no-default-features` check all pass.

## Open (not locally actionable or planned increments)

- [x] TYCHE-008 publication boundary slice: mark `tyche-core` as the sole
      publishable package and keep the Consus, Moirai, and facade adapters
      private; synchronize the release documentation and package metadata.

- [x] TYCHE-004: Morris elementary effects, Saltelli first- and total-order
      Sobol indices, and multi-output reports are implemented and covered by
      the sensitivity evidence suite; the provider backlog and CHANGELOG carry
      the implementation contract and verification record.
- [ ] TYCHE-005: versioned Consus study schema (metadata/payload schema and
      manifest-last completeness); durability waits for a Consus transaction
      contract.
- [x] TYCHE-006: consumer-migration documentation follow-ups. Kwavers PR 304
      is recorded in the README, CHANGELOG, backlog, and gap audit; the merged
      consumer delegates fixed Latin-hypercube and Sobol collocation designs
      to Tyche while retaining geometry mappings locally.
- [ ] TYCHE-008: crates.io Trusted Publishing release automation for
      `tyche-core` — owner-gated.

## Evidence boundary

The lower-case `checklist.md` is the sole tracked provider checklist. Atlas
root `backlog.md` owns cross-repository hashes, hosted-run identifiers, and
registration history; this file records only the local completion state and
points inward to that SSOT to avoid duplicated evidence.
