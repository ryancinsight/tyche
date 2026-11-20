# ADR 0008: Publish the Eunomia 0.12 study boundary

- Status: Accepted
- Date: 2026-11-20
- Class: `[major]`

## Context

Atlas applications exchange public Rust types across independently published crates. Cargo gives nominal types different identities when their package source or incompatible version differs. A fresh registry lock must select one identity for each type shared across the boundary.

## Decision

Publish tyche-core 0.5.0 from the verified provider source. Public sampling and statistics bounds use Eunomia 0.12. Direct consumers upgrade Eunomia together with Tyche. The core crate ships independently of the integration wrappers.

## Verification

Run package, API, and feature checks on the release revision. Verify the archived source and checksum and compile a consumer from an empty lockfile against the indexed versions.
