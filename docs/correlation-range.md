# Squared Pearson finalization

## Contract and red witness

For positive finite centered sums A and B and finite nonzero covariance C,
report finalization evaluates clamp(C²/(AB), 0, 1) in the selected native
scalar. Zero variance and zero covariance retain the existing zero policy.
For observations (-s,-s), (s,s), A=B=C=2s², so the exact index is one. The
baseline reports zero for s=2^±40 in binary32 and s=2^±300 in binary64,
although all three stored moments are exact finite nonzero values.

## Evaluation and rounding

The direct quotient is retained when both products are finite and at least
EPSILON. This conservative bound lies inside the normal range for supported
binary32/binary64 fields and preserves the original operation bits there.
No numerical tolerance or input is discarded by choosing the normalized path.

Otherwise decompose A=ma·2^ea, B=mb·2^eb and |C|=mc·2^ec using the landed
provider binary_exponent and scale_binary contract. All three mantissas lie
in [1,2), so their products lie in [1,4), and the quotient lies in (1/4,4).
Evaluate q=(mc·mc)/(ma·mb), then scale by 2^(2ec-ea-eb). Normalization is
exact; floating arithmetic stays in T. Both signed covariance values produce
the same squared index. Exponents for supported fields fit the i32 expression.

With unit roundoff u, three normal arithmetic roundings give relative error
bounded by gamma_3=3u/(1-3u) against the quotient of the stored moments.
Final scaling is exact for a normal result; gradual underflow can add half
a least-subnormal unit of absolute error. This is a forward-error bound,
not a correctly-rounded exact-rational promise. Near a subnormal rounding
midpoint the accumulated rounding can change the final bit, including
zero/nonzero classification. Classification is strict outside that error
interval; tests retain both midpoint witnesses and cases on either side
with a margin exceeding the bound. The existing clamp remains.

The accumulator layout, Welford updates, public interfaces, and provider
dependency graph are unchanged. Non-finite or already-overflowed moments and
observation-count overflow remain outside this repair's finite-moment
contract. No non-finite-input rejection or full-range accumulation claim is
made.

## Evidence

- Unchanged original four red tests plus the ordinary control.
- Native f32/f64 and F32/F64 scale grids, three parameters and two outputs.
- Orthogonal sign-column analytic indices 1/5, 4/5, 9/10 and 1/10.
- Integer-rational differential properties with shrinking and independent
  coordinate exponents; all reference integer products are exact.
- Subnormal stored moments, representable subnormal final indices, true
  underflow, negative correlation, constants and insufficient samples.
- The shared allocation instrument checks construction, updates and report.

Final qualification and independent review are tracked in the canonical
[backlog](../backlog.md#tyche-correlation-range-001).

## Measured report cost

An alternating nine-round ARM64/Rust 1.97 O3 harness measures the actual
public report of two parameters and two outputs. Ordinary medians change
from 3.69/5.04/4.17/5.49 ns to 7.00/9.83/7.83/9.59 ns for f32/f64/F32/F64;
all ordinary output bits match. Recovery medians are 34.45–35.46 ns, where
the original reports are incorrect. Moving recovery to a cold helper reduces
the initial ordinary overhead. These are one-host report timings, not an
application budget, throughput guarantee or peak-memory measurement.
