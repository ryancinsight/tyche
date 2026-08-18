//! Exact bounded-integer reduction shared by discrete samplers.

use core::num::NonZeroU64;

use super::{Counter, Seed, StreamAlgorithm, StreamDomain};

/// Reduce a typed counter word into a non-zero `u64` bound without modulo bias.
///
/// Rejected words advance only the retry coordinate for this logical address;
/// the caller's index and draw coordinates remain stable and random-accessible.
pub(in crate::sampling) fn bounded_integer<D: StreamDomain, A: StreamAlgorithm>(
    seed: Seed,
    index: u64,
    draw: u64,
    bound: NonZeroU64,
) -> u64 {
    let bound = bound.get();
    let rejection_threshold = bound.wrapping_neg() % bound;
    let mut attempt = 0_u64;

    loop {
        let word = Counter::<D, A>::word(seed, index, draw.wrapping_add(attempt));
        let product = u128::from(word) * u128::from(bound);
        let low = u64::try_from(product & u128::from(u64::MAX))
            .expect("invariant: product low half is bounded by u64");
        if low >= rejection_threshold {
            return u64::try_from(product >> 64)
                .expect("invariant: product high half is bounded by u64");
        }
        attempt = attempt.wrapping_add(1);
    }
}
