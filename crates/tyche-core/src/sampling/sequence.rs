//! Stateless counter-addressed pseudorandom words.

/// Reproducibility seed for an uncertainty study.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Seed(u64);

impl Seed {
    /// Construct a seed from its stable bit representation.
    #[must_use]
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    /// Stable bit representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }
}

/// Zero-sized `SplitMix64` counter mixer.
///
/// Tyche uses `SplitMix64` as a deterministic hash of a study seed and logical
/// coordinates, not as shared mutable RNG state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SplitMix64;

impl SplitMix64 {
    /// Mix a counter key into a pseudorandom word.
    #[must_use]
    pub const fn word(seed: Seed, sample: u64, stream: u64) -> u64 {
        let key = seed
            .bits()
            .wrapping_add(sample.wrapping_mul(0x9E37_79B9_7F4A_7C15))
            .wrapping_add(stream.wrapping_mul(0xD1B5_4A32_D192_ED03));
        Self::mix(key)
    }

    /// Map a counter key into `[0, 1)` using the high 53 bits.
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "every 53-bit mantissa value is exactly representable by f64"
    )]
    pub fn unit(seed: Seed, sample: u64, stream: u64) -> f64 {
        const SCALE: f64 = 1.0 / 9_007_199_254_740_992.0;
        ((Self::word(seed, sample, stream) >> 11) as f64) * SCALE
    }

    /// Map a counter key into the open unit interval `(0, 1)`.
    ///
    /// The half-unit offset prevents either logarithm endpoint from appearing
    /// in inverse-transform samplers.
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "every 53-bit mantissa value is exactly representable by f64"
    )]
    pub fn open_unit(seed: Seed, sample: u64, stream: u64) -> f64 {
        const SCALE: f64 = 1.0 / 9_007_199_254_740_992.0;
        (((Self::word(seed, sample, stream) >> 11) as f64) + 0.5) * SCALE
    }

    const fn mix(mut value: u64) -> u64 {
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }
}

/// Zero-sized standard-normal counter sampler.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StandardNormal;

impl StandardNormal {
    /// Return one standard-normal variate addressed by `(seed, sample, stream)`.
    ///
    /// Box-Muller consumes two independent counter streams. No mutable RNG
    /// state or cache is shared between trials.
    #[must_use]
    pub fn at(seed: Seed, sample: u64, stream: u64) -> f64 {
        use eunomia::{FloatElement, NumericElement, RealField};

        let first = SplitMix64::open_unit(seed, sample, stream.wrapping_mul(2));
        let second = SplitMix64::unit(seed, sample, stream.wrapping_mul(2).wrapping_add(1));
        let radius = <f64 as NumericElement>::sqrt(-2.0 * <f64 as FloatElement>::ln(first));
        radius * <f64 as FloatElement>::cos(<f64 as RealField>::TAU * second)
    }
}
