//! Native-format likelihood ratios without intermediate probability underflow.

use crate::sampling::SampleScalar;

/// Inputs come from validated weights; the proposal mass and both totals are
/// positive. The ordinary path retains the existing rounded operations.
#[inline]
pub(super) fn likelihood_ratio<T: SampleScalar>(
    target_mass: T,
    target_total: T,
    proposal_mass: T,
    proposal_total: T,
) -> T {
    if target_mass == T::ZERO {
        // Preserve a caller's negative zero, just as division does.
        return target_mass;
    }
    let target_probability = target_mass / target_total;
    let proposal_probability = proposal_mass / proposal_total;
    let (precision, max_exponent, normal_min) = format::<T>();
    let normal_min = T::from_f64(normal_min);
    if target_probability >= normal_min && proposal_probability >= normal_min {
        return target_probability / proposal_probability;
    }
    exact_ratio(
        target_mass,
        target_total,
        proposal_mass,
        proposal_total,
        precision,
        max_exponent,
    )
}

/// Compile-time metadata for the two sealed `SampleScalar` IEEE layouts.
/// These are representation constants, never conversions of an input value.
/// Keeping them constant removes exponent queries from the ordinary path.
const fn format<T: SampleScalar>() -> (i32, i32, f64) {
    match core::mem::size_of::<T>() {
        4 => (24, 127, 1.175_494_350_822_287_5e-38),
        8 => (53, 1023, f64::MIN_POSITIVE),
        _ => panic!("SampleScalar is sealed to binary32 and binary64"),
    }
}

/// Preserve the native significand in an integer bit field. Every floating
/// operation stays in T: binary scaling and the residual bit extraction are
/// exact. No floating value is widened to another scalar format.
fn significand<T: SampleScalar>(value: T, precision: i32) -> (u128, i32) {
    let exponent = value
        .binary_exponent()
        .expect("invariant: ratio factors are positive and finite");
    let mut fraction = value.scale_binary(-exponent) - T::ONE;
    let mut bits = 1_u128;
    let mut remaining = precision - 1;
    while remaining != 0 && fraction != T::ZERO {
        fraction += fraction;
        bits <<= 1;
        if fraction >= T::ONE {
            bits |= 1;
            fraction -= T::ONE;
        }
        remaining -= 1;
    }
    bits <<= remaining;
    debug_assert_eq!(fraction, T::ZERO);
    (bits, exponent)
}

/// The exceptional path rounds the exact ratio of four native-format factors.
/// Two at-most-53-bit significands need at most 106 bits per product. Ratio
/// normalization adds at most two bits; doubled remainders stay below 2^109.
/// Thus u128 stores representation bits exactly, without a wider float or
/// heap storage. Keeping the remainder makes overflow/subnormal ties explicit.
#[inline(never)]
#[cold]
fn exact_ratio<T: SampleScalar>(a: T, b: T, c: T, d: T, precision: i32, max_exponent: i32) -> T {
    // Exact common-factor cancellation reduces these cases to one native
    // division, including its correct overflow/subnormal rounding.
    if a == c {
        return d / b;
    }
    if b == d {
        return a / c;
    }
    let min_subnormal = 1 - max_exponent - (precision - 1);
    let (a, ae) = significand(a, precision);
    let (b, be) = significand(b, precision);
    let (c, ce) = significand(c, precision);
    let (d, de) = significand(d, precision);
    let mut numerator = a * d;
    let mut denominator = b * c;
    let mut exponent = ae + de - be - ce;
    while numerator < denominator {
        numerator <<= 1;
        exponent -= 1;
    }
    while numerator >= denominator * 2 {
        denominator <<= 1;
        exponent += 1;
    }
    if exponent > max_exponent {
        return T::INFINITY;
    }
    if exponent < min_subnormal - 1 {
        return T::ZERO;
    }
    if exponent == min_subnormal - 1 {
        // [half a subnormal, one subnormal): the exact half ties to even zero.
        return if numerator == denominator {
            T::ZERO
        } else {
            T::ONE.scale_binary(min_subnormal)
        };
    }
    let bits = precision.min(exponent - min_subnormal + 1);
    let mut quotient = 1_u128;
    let mut remainder = numerator - denominator;
    let mut remaining = bits - 1;
    while remaining != 0 && remainder != 0 {
        remainder <<= 1;
        quotient <<= 1;
        if remainder >= denominator {
            remainder -= denominator;
            quotient |= 1;
        }
        remaining -= 1;
    }
    quotient <<= remaining;
    let doubled = remainder * 2;
    if doubled > denominator || (doubled == denominator && quotient & 1 != 0) {
        quotient += 1;
    }
    // At most 2^53, exactly convertible in T. The value has already been
    // rounded to its final normal/subnormal grid, so scaling cannot round twice.
    T::from_integer(i64::try_from(quotient).expect("invariant: at most 53 bits"))
        .scale_binary(exponent - (bits - 1))
}
