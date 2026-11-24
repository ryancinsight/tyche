# ADR 0006: Tyche release scope for 1.0

- Status: Accepted
- Change class: minor
- Date: 2026-11-12
- Revised: 2026-11-24, ensemble-bagging seam delivered (ADR 0009)

## Context

Tyche (`tyche-core` sampling/UQ core + Moirai/Consus adapters + facade,
workspace 0.4.0) is the Atlas stack's reproducible uncertainty-study
provider: counter-addressed streams, random-access LHS and Sobol designs,
discrete/importance sampling, bootstrap indices, Morris/Saltelli
estimators, and versioned study execution. Sampling breadth (TYCHE-003)
and the multi-output sensitivity closure (TYCHE-004) are delivered, as is the
ensemble-bagging trainable-model seam ([ADR 0009](0009-retained-state-trainable-models.md));
two open scopes remain: the versioned study schema and the crates.io
release automation. No milestone ladder exists to 1.0.

## Decision

Adopt a three-milestone ladder to 1.0 recorded in `backlog/milestones.md`:

- **M-uq-closure** (0.5.0): the UQ surface is honest — a real
  trainable-model seam for ensemble bagging, replacing the score-only
  model that retains no trained state.
- **M-study-schema** (0.6.0): studies persist against a versioned
  metadata/payload schema with manifest-last completeness; durability
  itself waits on Consus transaction support (dependency, not ladder
  content).
- **M-1.0** (1.0.0): stable study release (authority-gated) with the
  `tyche-core` Trusted Publishing closure completed under release
  authority; adapters stay private (`publish = false`).

In scope: the trainable-model seam, the versioned schema, repository-side
publish dry runs. Out of scope: consumer confidence-interval policy
(stays consumer-owned); Consus durability (dependency); any registry
publication or account-setting claim without release authority (veto
point: M-1.0 never forces publication).

## Rejected alternatives

- Plan 1.0 as a single release: rejected because UQ closure and schema
  persistence are independently verifiable slices with an order between
  them (the schema versions what the estimators produce).
- Keep roadmap implicit in backlog sections: rejected because stack
  policy requires a runnable capability-oriented milestone ladder.

## Consequences

- Open backlog sections carry `Milestone:` assignments and name the exit
  check they serve.
- Milestones close only when exit checks pass on fetched default commits;
  1.0 release/publication remains authority-gated.
- This ADR is rewritten in place as scope evidence changes.
