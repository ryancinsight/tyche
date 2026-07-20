# ADR 0002: Preserve typed design failures

- Status: accepted
- Change class: major, architectural
- Date: 2026-07-20

## Context

`Design::sample_unit_into` requires `SampleIndexError`, but the error
constructor was crate-private. External implementations could not satisfy the
public trait's failure contract. `Study::sample` then collapsed the error to
`Option`, and the Moirai adapter asserted that every in-range index must
succeed. A malformed safe implementation therefore entered Moirai's contained
panic path instead of returning a domain failure.

## Decision

- Make `SampleIndexError::new(index, sample_count)` public.
- Return `Result<Sample<_, _>, SampleIndexError>` from `Study::sample`.
- Make `DispatchError` non-exhaustive and add `DesignContract`.
- Leave a rejected output slot when a design fails inside a scoped chunk, join
  every scoped job, then return the first missing logical index. This preserves
  caller-owned output storage and adds no success-path allocation.

## Public migration

Callers matching `Option` from `Study::sample` must match the typed `Result`
instead. Existing `.expect(...)` call sites remain source-compatible. Exhaustive
external matches over `DispatchError` must add a wildcard arm because the enum
is now non-exhaustive.

## Verification

An adversarial public `Design` declares three samples and rejects index one.
The Moirai adapter returns
`DispatchError::DesignContract { index: 1, sample_count: 3 }` after joining the
scope. The test also proves the failure is not collapsed into a scheduler
panic. Warning-denied Clippy and the complete Nextest suite cover the changed
core and adapter contracts.

## Rejected alternatives

- Retaining `Option` discards the public trait's typed failure.
- Adding a parallel `try_sample` method keeps two canonical sampling contracts.
- Panicking on a safe external implementation violates the library panic
  policy even when the runtime contains the unwind.
- Prevalidating every sample duplicates design evaluation and doubles the
  success-path work.
