# ADR 0005: Borrowed discrete importance sampling

- Status: Accepted
- Date: 2026-07-21
- Class: [minor] [arch]

## Context

TYCHE-003 still lacks discrete sampling. A complete provider boundary must
cover equal-probability categories, non-negative weighted categories, and
importance sampling from a proposal mass function without moving domain
responses or estimator policy into Tyche.

The inversion principle is specified by Devroye, *Non-Uniform Random Variate
Generation*, Chapter II, Section 2.1, Theorem 2.1. For a finite distribution,
the generalized inverse is the first category whose cumulative mass exceeds
the uniform threshold. Owen, *Monte Carlo Theory, Methods and Examples*,
Chapter 9, Section 9.1, Theorem 9.1 states the importance identity and the
support condition `q(x) > 0` whenever `f(x) p(x) != 0`.

## Decision

- `Categorical<A>` samples a validated non-zero category count with an exact
  multiply-high rejection reduction over the versioned counter stream.
- `DiscreteWeights<'a, T>` stores validated weights in `Cow<'a, [T]>`, so a
  caller chooses borrowed zero-copy storage or owned storage through one API.
- `WeightedCategorical<'a, T, A>` applies the finite inverse-CDF rule directly
  to borrowed weights. Construction is `O(K)`, each draw is `O(K)`, and draws
  allocate nothing.
- `DiscreteImportance<'a, T, A>` owns target and proposal weight contracts,
  validates equal cardinality and target support contained in proposal
  support, and returns `ImportanceSample<T>` with ratio `p_i / q_i`.
- The counter address is the complete logical draw identity. Rejection retries
  use the draw coordinate within a dedicated categorical stream domain.
- Scalar arithmetic remains native to `T: SampleScalar`; no distribution
  converts through a fixed precision.

## Proof obligations

Let raw non-negative masses be `w_i`, total mass `W > 0`, and
`U` uniform on `[0, 1)`. The weighted sampler returns the first `i` for which
`U W < sum_{j <= i} w_j`. Category `i` therefore owns an interval of length
`w_i`, giving probability `w_i / W`; zero-mass categories own empty intervals.

For target mass function `p`, proposal `q`, and any finite response `f`, the
support check makes `p_i / q_i` defined whenever `p_i f_i` can contribute.
Then

```text
E_q[f(I) p_I / q_I] = sum_i q_i f_i p_i / q_i = sum_i p_i f_i.
```

Uniform category reduction follows Lemire's multiply-high rejection rule. The
accepted 128-bit product partitions the accepted 64-bit words into equally
sized residue classes, so every category receives exactly the same number of
source words. `SplitMix64` maps the retry coordinate bijectively over `u64`,
so an accepted word exists for every non-zero category count.

## Alternatives rejected

- An alias table was rejected for this slice because it requires `O(K)` owned
  preprocessing storage and a second representation. No measured consumer
  workload currently justifies that memory/performance trade.
- A normalized-probability input was rejected because normalizing copies data
  and makes raw weights and probabilities parallel APIs.
- Dynamic distribution or algorithm dispatch was rejected because the
  implementor set and replay algorithm are compile-time choices.
- A Tyche-owned estimator closure was rejected because response evaluation,
  accumulation policy, and variance diagnostics belong to domain consumers
  and the existing statistics boundary.

## Consequences

The delivered representation is allocation-free for borrowed weights and for
all repeated draws. Weighted draws scale linearly with category count; a
future cumulative or alias strategy requires measured evidence and must enter
through the same distribution contract rather than clone importance logic.
Validation rejects empty, non-finite, negative, zero-total, mismatched, or
unsupported mass functions before sampling.

## References

- Luc Devroye, *Non-Uniform Random Variate Generation*, Chapter II,
  Section 2.1, Theorem 2.1:
  https://luc.devroye.org/chapter_two.pdf
- Art B. Owen, *Monte Carlo Theory, Methods and Examples*, Chapter 9,
  Section 9.1, Theorem 9.1:
  https://artowen.su.domains/mc/Ch-var-is.pdf
