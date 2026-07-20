# Tyche implementation backlog

## TYCHE-001 — Phase 0 core — implemented

- Reproducible design, study, ensemble, statistics, calibration, execution, and
  artifact-access vertical slice.

## TYCHE-002 — Public promotion — implemented

- Public `ryancinsight/tyche` origin and the Atlas gitlink are registered.

## TYCHE-006 — Consumer migrations — ready-review

- Helios PR 10 delegates reproducible normal noise to `StandardNormal`; its
  package-local suite passes 27/27 tests.
- CFDrs PR 299 delegates const-width Latin-hypercube designs to Tyche; its
  package-local suite passes 128/128 tests.
- Kwavers PR 298 delegates conformal calibration, moments, and correlation
  screening to Tyche; local suites pass 718/718 analysis and 1,251/1,251 solver
  tests. Hosted verification and merge remain.
- Moirai merge `91c802e` repairs the final-completion lifetime race exposed by
  Tyche's 257-item, seven-item-chunk adapter contract. The pinned Tyche
  workspace passes 16/16 tests, including the exact former access-violation
  case.

## TYCHE-003 — Sampling breadth — planned

- Runtime dimensions, Sobol, categorical, weighted, importance sampling, and
  versioned stream vectors.

## TYCHE-004 — UQ breadth — planned

- Reproducible bootstrap, Morris, true Saltelli Sobol, and multi-output reports.

## TYCHE-005 — Study schema — planned

- Versioned metadata/payload schema and manifest-last completeness; durability
  waits for Consus transaction support.
