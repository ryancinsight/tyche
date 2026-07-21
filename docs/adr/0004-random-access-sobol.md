# ADR 0004: Random-access Sobol designs

- Status: accepted
- Change class: minor, architectural
- Date: 2026-07-21

## Context

TYCHE-003 requires a low-discrepancy design that preserves Tyche's replay and
allocation contracts. The first concrete consumer selects its geometry
dimension at runtime and currently supports one through three coordinates. Its
local implementation builds a `Vec<Vec<f64>>`, allocates direction tables,
panics outside that dimension range, and maps a seed to an initial skip. A
seeded skip is neither scrambling nor an explicit sequence range, and dropping
initial Sobol points can discard the balance properties users expect.

The public boundary must serve compile-time study widths and runtime geometry
without cloning the sequence algorithm or adding dynamic dispatch.

## Decision

Tyche owns one const-generic coordinate kernel over the first three Algorithm
659 direction-number families. `Sobol<P, S>` implements the fixed-width
`Design<P>` contract. `RuntimeSobol<S>` validates a `SobolDimensions` value
once and dispatches each operation to `kernel::<1>`, `kernel::<2>`, or
`kernel::<3>`. `S: SobolScramble` is a sealed static policy:

- `Unscrambled` is a ZST exposing the canonical points;
- `DigitalShift<A>` stores one `Seed` and derives a distinct 32-bit XOR word
  per dimension through the explicit `A: StreamAlgorithm` contract.

`SobolRange` carries the first sequence index and a `NonZeroU32` count as
separate values. Construction rejects a range whose final index exceeds
`u32::MAX`. Starting at zero includes the origin; starting at one selects the
first non-origin point. A scramble seed never changes this range.

Single-point APIs write into caller-owned arrays or slices. The runtime
`sample_unit_rows_into` operation validates one row-major buffer, dispatches
once for the entire range, and fills it without a temporary point or
collection. The direction table is a 384-byte compile-time constant. No
direction table, point, row, or nested vector is allocated at runtime.

## Proof obligations

### Sequential and random-access equivalence

Let `g(n)=n XOR (n >> 1)` and let `v[d][j]` be direction `j` in dimension
`d`. Tyche evaluates

```text
x[n][d] = XOR(v[d][j] for every set bit j in g(n)) / 2^32.
```

For `n>0`, consecutive Gray codes differ at exactly
`j=trailing_zeros(n)`. Therefore the formula changes `x[n-1][d]` by exactly
`v[d][j]`, which is Bratley and Fox's sequential recurrence. Since both forms
start at zero, induction proves equality for every 32-bit point index. An
independently implemented sequential recurrence checks the first 1,024 points
bit-for-bit in all supported dimensions.

### Unit-hypercube bounds

The XOR numerator is a `u32`, so it lies in `[0, 2^32-1]`. Multiplication by
`2^-32` therefore produces a binary-exact `f64` in `[0,1)`. XOR with a digital
shift remains a `u32` and preserves the same bound.

### Dyadic projection stratification

For an origin-aligned prefix of `2^m` points, `m<=31`, the Gray-code map is a
bijection over the `m` input bits. Each supported direction matrix restricted
to its first `m` output bits is triangular over `GF(2)` with a one on every
diagonal entry because each direction numerator is odd before alignment.
That matrix is invertible, so every `m`-bit coordinate prefix, and therefore
every one-dimensional dyadic stratum, occurs once. XOR by a fixed digital
shift permutes those prefixes and preserves their counts. The test suite checks
all three projections at `m=8` for both policies. This is a projection law, not
a claim that arbitrary counts or skipped ranges retain a full `(t,m,s)` net.

### Replay and failure atomicity

Coordinates depend only on the explicit algorithm version, shift seed,
dimension, and sequence index. Evaluation order is absent, so forward and
reverse access are bitwise identical. Output dimensions and indices are
validated before the corresponding buffer is written. The row-major operation
validates total length before its first row, so every reported length failure
leaves caller memory unchanged.

## Performance and memory evidence

Criterion 0.8.2 ran on an Intel Core Ultra 9 285K, Windows x86-64 GNU, Rust
1.95.0/LLVM 22.1.3. The initial correct kernel rediscovered the active Gray
bits once per dimension. Hoisting that invariant discovery and applying each
active direction across the const-width numerator array produced:

| Operation | Before median (95% interval) | Hoisted median (95% interval) | Criterion change estimate |
|---|---:|---:|---:|
| fixed width 3 | 8.2945 ns (8.1996–8.3946) | 7.0705 ns (6.9653–7.2056) | -14.03% |
| runtime width 3 | 9.3333 ns (9.1384–9.5251) | 6.9186 ns (6.7497–7.1139) | -27.78% |
| runtime 4,096-row fill | 30.429 µs (30.050–30.825) | 13.743 µs (13.698–13.790) | -54.43% |

The final fixed and runtime single-point intervals overlap, so the instrument
detects no remaining runtime-dispatch tax at width three. The batched path is
3.35 ns per point and 894.1 million coordinates per second in this controlled
run. These measurements establish this machine and workload only.

The allocation regression covers fixed points, runtime points, and a complete
runtime row-major fill inside one instrumented post-construction region and
reports zero allocations, reallocations, or deallocations. `Unscrambled` is a
ZST. A fixed unscrambled design occupies only its `SobolRange`; a runtime
unscrambled design adds only `SobolDimensions`; `DigitalShift<SplitMix64>`
occupies one `Seed`.

The public validation and fixed/runtime specialization surface increases
development IR from 505 lines/56 copies after ADR 0003 to 859 lines/94 copies.
The linked release example changes from 619.5 KiB to 620.0 KiB of `.text`
(+0.08%), with `tyche_core` attribution changing from 15.5 KiB to 16.1 KiB;
the stripped file remains 1.2 MiB. A focused simplification of the cold range
error formatter did not change the linked result. The retained 0.5 KiB is the
bounded artifact cost of the new public validation paths; the direction table
and uninstantiated width variants are removed from this non-Sobol example.

## Compatibility and migration

This is an additive public API. A runtime consumer replaces nested point
storage with one destination matrix and calls `sample_unit_rows_into`. It must
choose an explicit range and an explicit `Unscrambled` or
`DigitalShift<SplitMix64>` policy. A former seed-derived skip becomes a real
shift seed only when randomized replay is intended; sequence continuation uses
`SobolRange::start` instead.

## Rejected alternatives

- Preserve seed-as-skip: conflates two independent contracts and obscures the
  known risk of dropping initial points.
- Allocate direction tables or nested points: duplicates static data and makes
  memory scale beyond the caller's required result matrix.
- Accept arbitrary dimensions with partial data: silently fabricates sequence
  quality. The current verified consumer boundary is one through three;
  extending it requires authoritative direction data and generic tests.
- Runtime trait objects: the implementor set is closed and dimensions dispatch
  at an operation boundary, so enum-like const-kernel selection is sufficient.
- Separate fixed and runtime algorithms: duplicates the mathematical SSOT and
  permits their vectors to drift.

## References

- Bratley and Fox, “Algorithm 659: Implementing Sobol's Quasirandom Sequence
  Generator,” *ACM Transactions on Mathematical Software* 14(1), 88–100
  (1988); archived algorithm entry at <https://netlib.org/toms/>.
- Joe and Kuo, “Notes on Generating Sobol' Sequences,” Sections 1–4,
  <https://web.maths.unsw.edu.au/~fkuo/sobol/joe-kuo-notes.pdf>.
