# ADR 0001: Reproducible uncertainty-study boundary

- Status: accepted
- Change class: minor, architectural
- Date: 2026-07-20

## Decision

Use four inward-dependent crates: a `no_std + alloc` core, borrowed Moirai and
Consus adapters, and a curated facade. Core owns validated spaces,
counter-addressed streams, random-access LHS, GAT model responses, ordered
moments, explicit variance policies, honest correlation screening, and
corrected conformal calibration. It owns no runtime, store, array, or physics.

`Study::sample` preserves `SampleIndexError`; public `Design` implementations
can construct that typed failure. Moirai dispatch writes caller-owned disjoint
chunks. Model errors remain indexed values, while scheduler and malformed
design-contract failures are separate. Consus adaptation validates relative
keys before delegation and persists only after dispatch.

## Proof obligations

### Latin-hypercube stratification

Let `gcd(a,n)=1` and `pi(i)=a*i+b (mod n)`. If `pi(i)=pi(j)`, then
`a(i-j)=0 (mod n)`. Multiplying by the modular inverse of `a` gives `i=j`.
Thus `pi` is a bijection. Since `floor(n*(pi(i)+u_i)/n)=pi(i)` for
`0<=u_i<1`, every stratum occurs exactly once.

### Interval preservation

For finite `l<h` and `0<=u<1`, `0<=u(h-l)<h-l`; adding `l` yields
`l<=l+u(h-l)<h`. Tests use bounded scientific ranges and do not overclaim
universal IEEE endpoint exactness.

### Replay invariance

Coordinates depend only on algorithm version, seed, logical index, and
dimension. Each trial writes the same logical slot. Scheduling appears in
neither input nor destination, so a scheduling permutation changes only
evaluation time. Ordered serial summaries keep floating association invariant
across worker counts.

### Welford invariant

With `delta=x_n-m_(n-1)`, update
`m_n=m_(n-1)+delta/n` and
`M2_n=M2_(n-1)+delta*(x_n-m_n)`. Substitution proves the arithmetic-mean
identity. Expanding old deviations around `m_n` cancels the linear cross term
and yields the centered-sum identity. Population variance is `M2/n`; sample
variance is `M2/(n-1)` for `n>=2`.

### Correlation bound

For centered vectors, `r²=<x,y>²/(||x||²||y||²)`. Cauchy-Schwarz gives
`0<=r²<=1`. This is screening, not a first-order/total Sobol decomposition.

### Conformal coverage

For `n` exchangeable calibration scores and one future score, the rank among
`n+1` values is uniform under random tie breaking. Choosing
`ceil((n+1)(1-alpha))`, capped at `n`, excludes at most an alpha fraction of
ranks, establishing marginal coverage subject to exchangeability. Tyche embeds
the structural count into `T` before multiplying by `(1-alpha)` and applying
`ceil`, so each monomorphization performs the rank arithmetic in its native
scalar precision. For a validated nondecreasing score slice, scanning integer
positions selects the first position not less than that rank; if scalar
rounding places it above `n`, selecting the final score implements the stated
cap. The borrowed sorted path therefore performs no allocation or mutation.

### Persistence boundary

A future manifest-last schema proves logical completeness only. Consus `Store`
has no flush or transaction contract, so Tyche makes no crash-durability claim.

## Representation consequences

- `Cow` is confined to metadata; samples remain fixed arrays or slices.
- GAT responses can borrow and are consumed by static reducers.
- Designs, models, reducers, scalar precision, const widths, and policies
  monomorphize.
- Counter/distribution and variance markers are ZSTs.
- Core sampling and statistics allocate nothing.

## Rejected alternatives

- Mutable per-worker RNG: schedule-dependent replay.
- Stored LHS matrix: unnecessary `O(samples*parameters)` storage.
- Tyche thread pool or file format: violates Moirai/Consus ownership.
- Worker-callback persistence: unordered mutable store access.
- One variance default: consumers intentionally require both conventions.
- Calling correlation Sobol: false for interacting models.
- Dynamic model objects: boxing and lost borrowing are unnecessary.

## Verification

Stratification, bounds, bitwise replay, stream domains, Welford/Chan oracles,
variance denominators, correlation, conformal rank, GAT/Cow identity, ZST and
allocation checks, Moirai index preservation and malformed-design rejection,
Consus byte roundtrip, safe keys, no-std, Clippy, tests, rustdoc, example, and
supply-chain gates.

## References

- McKay, Beckman, and Conover, “A Comparison of Three Methods for Selecting
  Values of Input Variables in the Analysis of Output from a Computer Code,”
  *Technometrics* 21(2), 239–245 (1979),
  <https://doi.org/10.1080/00401706.1979.10489755>.
- Welford, “Note on a Method for Calculating Corrected Sums of Squares and
  Products,” *Technometrics* 4(3), 419–420 (1962),
  <https://doi.org/10.1080/00401706.1962.10490022>.
- Angelopoulos and Bates, “A Gentle Introduction to Conformal Prediction and
  Distribution-Free Uncertainty Quantification,” Section 1.1 and Appendix D,
  arXiv:2107.07511v6 (2022), <https://arxiv.org/abs/2107.07511>.
