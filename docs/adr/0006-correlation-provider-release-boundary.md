# ADR 0006: Correlation repair and the provider release boundary

- Status: Proposed
- Date: 2026-11-04

## Context

The private correlation finalization repair is patch-class relative to current
main. However, published tyche-core 0.4.0 contains Eunomia 0.10, while current
main already requires Eunomia 0.11. A client using Eunomia 0.10 F64 builds and
runs with the published package and fails its RealField bound with the
preliminary normalized candidate, whose provider requirement is inherited
from current main. Patch promotion would misstate that public
nominal-type boundary. Fresh Git resolution also fails because the floating
provider default is now Eunomia 0.12.

Independent design signoff supports the proposed 0.5.0 classification before
its metadata implementation. Status remains Proposed because source graph,
consumer migration and release acceptance remain held.

## Decision proposed

Classify the next package release as 0.5.0, with an explicit Breaking entry
for the already-landed provider identity change and a migration guide. Keep
the correlation algorithm generic over the existing native RealField, with
no wrapper shims or widening. Its mathematical scope and error bound live in
[the numerical contract](../correlation-range.md).

Do not merge or publish this candidate until existing provider/cohort ownership
resolves fresh source compatibility and the final release graph is qualified.
Preserve Tyche importance-sampling PR 110, provider-lock PR 115, and hook PR 116.
Changing the version does not satisfy those gates or authorize taking over
those branches. The private repair can receive independent source review and
draft delivery while release acceptance remains held.

## Alternatives

- A 0.4.1 release hides the demonstrated wrapper-type break and is rejected.
- Back-pinning to published Eunomia 0.10 discards the landed compatibility
  direction and is rejected.
- Implementing a duplicate provider/cohort migration bypasses existing
  ownership and is rejected; existing owners retain that sequence.

## Acceptance

Independent signoff precedes changing the proposed own release boundary.
All original correctness regressions, full source gates, package tests,
compatibility evidence, normal hooks and final source review remain required.
Consumer migration and exact hosted/release/source receipts precede release
acceptance. No registry or deployment claim follows from this ADR.
