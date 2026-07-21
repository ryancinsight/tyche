# ADR 0003: Domain-separated counter schedule

- Status: accepted
- Change class: major, architectural
- Date: 2026-07-21

## Context

The original public stream accepted an untyped `(seed, sample, stream)` tuple.
Latin-hypercube stride, offset, and jitter code assigned different meanings to
overlapping tuples. For at least three dimensions, jitter `(sample=0,
dimension=d)` reused the stride word for dimension `d+2`, and jitter
`(sample=1, dimension=d)` reused that dimension's offset word. Normal draws
also occupied the same namespace. ADR 0001 required an algorithm version for
replay, but neither the type surface nor exact vectors identified one.

## Decision

Every counter is `Counter<D, A>`, where `D: StreamDomain` is a sealed ZST and
`A: StreamAlgorithm` is an explicitly selected sealed algorithm ZST. Tyche's
first schedule is `SplitMix64`, carrying nonzero `StreamVersion(1)`. Public
downstream domains use `UserDomain<TAG>`; the const tag is part of persisted
replay identity. Latin-hypercube stride, offset, and jitter plus normal radius
and angle own distinct internal tags. Compile-time assertions reject duplicate
internal tags.

For seed `s`, domain tag `t`, logical index `i`, and draw `r`, the schedule is

```text
key = s + t + i * 0x9e3779b97f4a7c15
            + r * 0xd1b54a32d192ed03  (mod 2^64)
word = Mix13(key)
```

Both steps are odd. The finalizer is Stafford's Mix13 permutation. Raw offset
arithmetic is no longer a public semantic contract. `Counter`,
`LatinHypercube`, and `StandardNormal` have no default algorithm parameter, so
callers select the replay contract in the type.

`SampleScalar` performs unit conversion at native scalar precision. `f32`
uses 24 counter bits and `f64` uses 53; generic normal sampling no longer
creates a double-precision unit value and narrows it. The open-unit mapping
sends the otherwise-zero cell to its positive half-cell midpoint. Box-Muller
uses distinct radius and angle domains.

## Proof obligations

### Coordinate separation

An xor-right-shift is invertible by reconstructing high bits first. Each
Mix13 multiplier is odd and therefore has a multiplicative inverse modulo
`2^64`; Mix13 is a composition of bijections. Modular addition is also a
bijection. Thus, with seed, index, and draw fixed, unequal domain tags produce
unequal words. With all but index fixed, the odd index step makes the index
map bijective; the same argument applies to draw. A map from all coordinate
tuples into 64 bits cannot be globally injective, so Tyche makes no stronger
collision claim.

### Unit intervals

For native significand width `p`, let `k` be the selected high-bit integer,
`0 <= k < 2^p`. Then `k * 2^-p` lies in `[0,1)`. The open mapping is
`2^-(p+1)` for `k=0` and otherwise `k * 2^-p`, hence lies strictly in `(0,1)`.

### Normal transform

For counter-derived `u1` in `(0,1)` and `u2` in `[0,1)`, Box-Muller computes
`sqrt(-2 ln u1) cos(2 pi u2)`. Separate typed domains prevent the radius and
angle coordinates from aliasing at equal logical coordinates. The bijection
proof establishes addressing, not statistical independence; distribution
quality remains empirical evidence.

## Performance and memory evidence

Criterion 0.8.2 ran on an Intel Core Ultra 9 285K, Windows x86-64 GNU, Rust
1.95.0/LLVM 22.1.3. The baseline is `origin/main` `94d3c34`; both revisions use
the same committed benchmark and shared release profile.

| Operation | Baseline median (95% interval) | Typed schedule | Change |
|---|---:|---:|---:|
| word | 573.05 ps (568.72–580.57) | 568.44 ps (567.33–569.60) | within noise |
| normal | 14.284 ns (14.262–14.306) | 14.315 ns (14.289–14.358) | no detected change, `p=0.81` |
| width-8 LHS | 13.192 ns (12.945–13.446) | 12.356 ns (12.332–12.383) | 7.87% lower median, `p<0.01` |

The initially evaluated nested schedule applied Mix13 three times. It regressed
word latency by 233% and normal latency by 26.3%, so the measurement falsified
that design. One finalizer over odd-stepped typed coordinates provides the
required separation without the regression. All algorithm/domain policies
remain ZSTs, and the allocation regression test covers counter, normal, LHS,
and moments in one post-construction region.

`cargo llvm-lines -p tyche-core --lib` increases emitted development IR from
469 lines/46 copies to 505 lines/56 copies for the additional public scalar and
version contracts. `cargo bloat` on the release `reproducible_study` example is
unchanged after linking: 619.5 KiB `.text`, 1.2 MiB file size, and 15.5 KiB
attributed to `tyche_core` on both revisions. The explicit policies therefore
add checked surface without increasing this shipped artifact.

## Compatibility and migration

This intentionally changes seeded LHS and standard-normal bit patterns. It
also replaces public untyped calls:

- `SplitMix64::word(seed, index, draw)` becomes
  `Counter::<UserDomain<TAG>, SplitMix64>::word(seed, index, draw)`;
- `StandardNormal::<T>` becomes `StandardNormal::<T, SplitMix64>`;
- `LatinHypercube::<P>` becomes `LatinHypercube::<P, SplitMix64>`.

Consumers must choose a stable domain tag and update bitwise contract vectors.
No compatibility alias or old stream remains.

## Rejected alternatives

- Keep raw tuples and document ranges: the existing alias was valid under that
  contract, and future samplers would repeat the risk.
- Runtime string/enum domains: adds dispatch or hashing to every draw.
- Three nested Mix13 calls: correct addressing but measured regression.
- Preserve old outputs: preserves the identified cross-algorithm correlation
  and leaves no explicit replay version.

## References

- Steele, Lea, and Flood, “Fast Splittable Pseudorandom Number Generators,”
  OOPSLA 2014, Section 2.4 and Figures 16–17,
  <https://gee.cs.oswego.edu/dl/papers/oopsla14.pdf>.
- Box and Muller, “A Note on the Generation of Random Normal Deviates,”
  *Annals of Mathematical Statistics* 29(2), 610–611 (1958),
  <https://doi.org/10.1214/aoms/1177706645>.
