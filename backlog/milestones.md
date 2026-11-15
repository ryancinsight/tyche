# Milestone ladder ([ADR 0006](../docs/adr/0006-release-scope-1-0.md))
<a id="M-uq-closure"></a> **M-uq-closure** (0.5.0) needs: (none) — the UQ surface is honest.
- exit: pass provider UQ gates (`cargo test -p tyche-core --locked --all-features` passes, including the trainable-model seam ensemble-bagging case over a retained trainable model).
- standing: no open correctness item.
<a id="M-study-schema"></a> **M-study-schema** (0.6.0) needs: M-uq-closure — studies persist against a versioned schema.
- exit: pass schema round-trip gates (versioned metadata/payload schema with manifest-last completeness; `cargo test --workspace --locked --all-features` passes).
- standing: no open correctness item.
<a id="M-1.0"></a> **M-1.0** (1.0.0) needs: M-study-schema — stable study release (authority-gated).
- standing: `cargo fmt --all -- --check`, `cargo clippy --workspace --locked --all-targets --all-features -- -D warnings`, `cargo test --workspace --locked --all-features`, `cargo test --workspace --locked --doc --all-features`, and `cargo doc --workspace --locked --no-deps --all-features` pass; SemVer and publish dry-run gates required for declared releases pass; README and rustdoc are current; no open correctness item.
