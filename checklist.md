# Tyche checklist

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
- [x] TYCHE-004 (first increment): deterministic random-access bootstrap index
      generation (`Bootstrap::<SplitMix64>`) with validated sizes, caller-owned
      fill, and the shared multiply-high reducer; Kwavers elastography
      percentile bootstrap delegates to it.
- [x] Re-release at 0.2.0 onto Eunomia 0.8 (`45354e6`), facade packaging and
      documentation (`ee3a52b`), and CI package-selector fixes.
- [x] Canonical provider gates at the audited revision (2026-08-12): strict
      all-target check with `-D warnings`, Clippy `-D warnings`
      (all-targets/all-features), Nextest 50/50 (all-features), 17/17
      doctests, and the `--no-default-features` check all pass.

## Open (not locally actionable or planned increments)

- [ ] TYCHE-004 remaining increments: genuine Morris and Saltelli Sobol
      estimators plus multi-output reports.
- [ ] TYCHE-005: versioned Consus study schema (metadata/payload schema and
      manifest-last completeness); durability waits for a Consus transaction
      contract.
- [ ] TYCHE-006: remaining consumer-migration documentation follow-ups
      (Kwavers PR 304 record in README/CHANGELOG).
- [ ] TYCHE-008: crates.io Trusted Publishing release automation for
      `tyche-core` — owner-gated.

## Evidence boundary

The lower-case `checklist.md` is the sole tracked provider checklist. Atlas
root `backlog.md` owns cross-repository hashes, hosted-run identifiers, and
registration history; this file records only the local completion state and
points inward to that SSOT to avoid duplicated evidence.
