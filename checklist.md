# Tyche checklist

- [x] ATLAS-TYCHE-WORKFLOW-PIN-2026-08-20: refresh the Pages caller to Atlas
      `20c9398`, preserving the executable `tyche-core` book gate. Commit
      `04c4400` and `5782c69` pass workflow-shape, strict-link, and mdBook build checks;
      Tyche PR [#34](https://github.com/ryancinsight/tyche/pull/34) is open.
- [ ] Collect exact-head hosted book evidence before advancing the Atlas
      gitlink.

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
