//! Native-precision unit-interval conversion.

use eunomia::RealField;

mod private {
    pub trait Sealed {}
}

/// A real scalar with an exact native-precision counter-word conversion.
///
/// Tyche implements this sealed trait for every primitive real field supported
/// by Eunomia. Conversion retains the scalar's own significand width; generic
/// distributions do not widen through another floating-point type.
pub trait SampleScalar: private::Sealed + RealField {
    /// Convert a word to the discrete uniform grid in `[0, 1)`.
    fn unit_from_word(word: u64) -> Self;

    /// Convert a word to a discrete uniform grid in `(0, 1)`.
    ///
    /// The word mapping sends the otherwise-zero cell to its positive
    /// half-cell midpoint. Every other cell retains the closed-open mapping.
    fn open_unit_from_word(word: u64) -> Self;
}

/// Implement [`SampleScalar`] for one primitive with an explicit high-word
/// shift and scale.
///
/// `$shift` selects the scalar's significand-width high bits and `$scale` is
/// their reciprocal, so the closed-open mapping stays exact in `$t`; `$half`
/// is the positive half-cell midpoint used by `open_unit_from_word`.
macro_rules! sample_scalar {
    ($t:ty, $shift:expr, $scale:expr, $half:expr, $reason:literal) => {
        impl private::Sealed for $t {}

        impl SampleScalar for $t {
            #[expect(clippy::cast_precision_loss, reason = $reason)]
            fn unit_from_word(word: u64) -> Self {
                const SCALE: $t = $scale;
                ((word >> $shift) as $t) * SCALE
            }

            fn open_unit_from_word(word: u64) -> Self {
                let value = Self::unit_from_word(word);
                if value == 0.0 { $half } else { value }
            }
        }
    };
}

sample_scalar!(
    f32,
    40,
    1.0 / 16_777_216.0,
    0.5 / 16_777_216.0,
    "the selected 24 bits are exactly representable by f32"
);

sample_scalar!(
    f64,
    11,
    1.0 / 9_007_199_254_740_992.0,
    0.5 / 9_007_199_254_740_992.0,
    "the selected 53 bits are exactly representable by f64"
);
