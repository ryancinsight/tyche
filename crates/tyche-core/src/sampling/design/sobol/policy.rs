//! Compile-time Sobol scrambling policies.

use core::marker::PhantomData;

use crate::sampling::counter::{Counter, Seed, SobolDigitalShift, StreamAlgorithm};

mod private {
    pub trait Sealed {
        fn shift(&self, dimension: usize) -> u32;
    }
}

/// A statically dispatched Sobol scrambling policy.
///
/// The sealed implementations are [`Unscrambled`] and [`DigitalShift`]. The
/// policy is selected once at the design boundary and monomorphizes the whole
/// coordinate kernel without a per-coordinate vtable branch.
pub trait SobolScramble: private::Sealed + Copy {}

/// Preserve the canonical, unshifted Sobol points.
#[must_use]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Unscrambled;

impl private::Sealed for Unscrambled {
    fn shift(&self, _dimension: usize) -> u32 {
        0
    }
}

impl SobolScramble for Unscrambled {}

/// Apply one deterministic base-two digital shift per dimension.
///
/// XOR by a fixed 32-bit word is a bijection of dyadic cells at every
/// representable resolution. It therefore preserves the point count in each
/// such cell while reproducibly moving the coordinates. This policy is a
/// digital shift, not an Owen nested scramble.
///
/// # Examples
///
/// ```
/// use tyche_core::{DigitalShift, Seed, SplitMix64};
///
/// let shift = DigitalShift::<SplitMix64>::new(Seed::new(17));
/// assert_eq!(shift.seed(), Seed::new(17));
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigitalShift<A> {
    seed: Seed,
    algorithm: PhantomData<A>,
}

impl<A: StreamAlgorithm> DigitalShift<A> {
    /// Construct a versioned digital shift from a study seed.
    pub const fn new(seed: Seed) -> Self {
        Self {
            seed,
            algorithm: PhantomData,
        }
    }

    /// Stable shift seed.
    pub const fn seed(self) -> Seed {
        self.seed
    }
}

impl<A: StreamAlgorithm> private::Sealed for DigitalShift<A> {
    fn shift(&self, dimension: usize) -> u32 {
        let Ok(dimension) = u64::try_from(dimension) else {
            unreachable!("invariant: supported Sobol dimensions fit in u64");
        };
        let high = Counter::<SobolDigitalShift, A>::word(self.seed, dimension, 0) >> 32;
        match u32::try_from(high) {
            Ok(value) => value,
            Err(_) => unreachable!("invariant: the high half of a u64 fits in u32"),
        }
    }
}

impl<A: StreamAlgorithm> SobolScramble for DigitalShift<A> {}

pub(super) fn shift<S: SobolScramble>(policy: &S, dimension: usize) -> u32 {
    private::Sealed::shift(policy, dimension)
}
