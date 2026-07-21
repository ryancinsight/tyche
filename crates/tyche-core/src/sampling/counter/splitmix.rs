//! Domain-separated `SplitMix64` counter schedule.

use core::{marker::PhantomData, num::NonZeroU32};

use super::{SampleScalar, Seed, StreamDomain};

mod private {
    use super::{Seed, StreamDomain};

    pub trait Sealed {}

    pub trait Generate {
        fn word<D: StreamDomain>(seed: Seed, index: u64, draw: u64) -> u64;
    }
}

/// Stable serialized identity of a counter-stream schedule.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StreamVersion(NonZeroU32);

impl StreamVersion {
    /// Construct a stream-version value.
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        match NonZeroU32::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Stable integer representation.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

/// Zero-sized `SplitMix64` mixing policy.
///
/// The finalizer is Stafford's Mix13 permutation as described in Section 2.4
/// of Steele, Lea, and Flood, *Fast Splittable Pseudorandom Number
/// Generators* (OOPSLA 2014). Tyche applies that permutation at each typed
/// domain-tagged `(index, draw)` key; it does not maintain mutable RNG state.
#[must_use]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SplitMix64;

impl SplitMix64 {
    /// Version of Tyche's domain-separated counter schedule.
    pub const VERSION: StreamVersion = StreamVersion(NonZeroU32::MIN);

    const fn mix(mut value: u64) -> u64 {
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }
}

/// A compile-time counter-stream algorithm and replay version.
///
/// The trait is sealed: new algorithms land in Tyche with known-answer vectors
/// and a distinct semantic type. Callers select the algorithm explicitly in
/// [`Counter`] and every distribution or design that consumes it. For each
/// fixed seed, domain, and index, an implementation must map the complete
/// `u64` draw coordinate bijectively over `u64`; exact categorical rejection
/// reduction relies on an accepted word existing in that finite permutation.
pub trait StreamAlgorithm: private::Sealed + private::Generate + Copy {
    /// Stable replay version.
    const VERSION: StreamVersion;
}

impl private::Sealed for SplitMix64 {}

impl private::Generate for SplitMix64 {
    fn word<D: StreamDomain>(seed: Seed, index: u64, draw: u64) -> u64 {
        const INDEX_STEP: u64 = 0x9E37_79B9_7F4A_7C15;
        const DRAW_STEP: u64 = 0xD1B5_4A32_D192_ED03;

        let key = seed
            .bits()
            .wrapping_add(D::TAG)
            .wrapping_add(index.wrapping_mul(INDEX_STEP))
            .wrapping_add(draw.wrapping_mul(DRAW_STEP));
        Self::mix(key)
    }
}

impl StreamAlgorithm for SplitMix64 {
    const VERSION: StreamVersion = Self::VERSION;
}

/// Zero-sized counter stream in compile-time domain `D`.
///
/// Domain selection is monomorphized. For a fixed seed and any two distinct
/// domain tags, equal `(index, draw)` coordinates produce distinct words
/// because modular addition and Mix13 are bijections on `u64`. The index and
/// draw steps are odd, so either coordinate is also bijective while the other
/// coordinates remain fixed.
#[must_use]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counter<D, A> {
    _policy: PhantomData<(D, A)>,
}

impl<D: StreamDomain, A: StreamAlgorithm> Counter<D, A> {
    /// Version of the counter schedule.
    pub const VERSION: StreamVersion = A::VERSION;

    /// Mix a typed logical coordinate into a pseudorandom word.
    #[must_use]
    pub fn word(seed: Seed, index: u64, draw: u64) -> u64 {
        <A as private::Generate>::word::<D>(seed, index, draw)
    }

    /// Map a logical coordinate into `T`'s native grid in `[0, 1)`.
    #[must_use]
    pub fn unit<T: SampleScalar>(seed: Seed, index: u64, draw: u64) -> T {
        T::unit_from_word(Self::word(seed, index, draw))
    }

    /// Map a logical coordinate into the open unit interval `(0, 1)`.
    ///
    /// The native scalar mapping moves the otherwise-zero cell to a positive
    /// half-cell midpoint, preventing either logarithm endpoint from appearing
    /// in inverse-transform samplers.
    #[must_use]
    pub fn open_unit<T: SampleScalar>(seed: Seed, index: u64, draw: u64) -> T {
        T::open_unit_from_word(Self::word(seed, index, draw))
    }
}
