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

impl private::Sealed for f32 {}

impl SampleScalar for f32 {
    #[expect(
        clippy::cast_precision_loss,
        reason = "the selected 24 bits are exactly representable by f32"
    )]
    fn unit_from_word(word: u64) -> Self {
        const SCALE: f32 = 1.0 / 16_777_216.0;
        ((word >> 40) as u32) as f32 * SCALE
    }

    fn open_unit_from_word(word: u64) -> Self {
        let value = Self::unit_from_word(word);
        if value == 0.0 {
            0.5 / 16_777_216.0
        } else {
            value
        }
    }
}

impl private::Sealed for f64 {}

impl SampleScalar for f64 {
    #[expect(
        clippy::cast_precision_loss,
        reason = "the selected 53 bits are exactly representable by f64"
    )]
    fn unit_from_word(word: u64) -> Self {
        const SCALE: f64 = 1.0 / 9_007_199_254_740_992.0;
        ((word >> 11) as f64) * SCALE
    }

    fn open_unit_from_word(word: u64) -> Self {
        let value = Self::unit_from_word(word);
        if value == 0.0 {
            0.5 / 9_007_199_254_740_992.0
        } else {
            value
        }
    }
}
