//! Independent finite-moment witnesses at gradual-underflow boundaries.

use eunomia::{F32, F64, FloatElement, RealField};

use super::squared_correlation;

fn assert_midpoint_bound<T: RealField>(left: T, right: T, covariance: T, exponent: i32, units: T) {
    let actual = squared_correlation(covariance, left, right);
    let actual_units = actual.scale_binary(-exponent);
    // These exact-rational witnesses are within one native epsilon of 1/2
    // or 3/2 least-subnormal units. Include that reference-rounding envelope
    // and gamma_3, separately from the final half-unit scaling error.
    let bound = T::from_f64(0.5) + T::from_f64(8.0) * T::EPSILON;
    assert!((actual_units - units).abs() <= bound);
}

#[test]
fn reviewer_binary32_midpoint_witnesses_obey_forward_error_contract() {
    for (a, b, c, units) in [
        (0x5e80_0000, 0x5eaa_aab3, 0x3980_0003, 1.5),
        (0x6500_0001, 0x64ff_fffe, 0x3f80_0000, 0.5),
    ] {
        let (a, b, c) = (f32::from_bits(a), f32::from_bits(b), f32::from_bits(c));
        assert_midpoint_bound(a, b, c, -149, units);
        assert_midpoint_bound(
            F32::from_f32(a),
            F32::from_f32(b),
            F32::from_f32(c),
            -149,
            F32::from_f32(units),
        );
    }
}

#[test]
fn reviewer_binary64_midpoint_witnesses_obey_forward_error_contract() {
    for (a, b, c, units) in [
        (
            0x5fc0_0000_0000_0000,
            0x5fd5_5555_5555_555b,
            0x3e40_0000_0000_0002,
            1.5,
        ),
        (
            0x6190_0000_0000_0001,
            0x617f_ffff_ffff_fffe,
            0x3ff0_0000_0000_0000,
            0.5,
        ),
    ] {
        let (a, b, c) = (f64::from_bits(a), f64::from_bits(b), f64::from_bits(c));
        assert_midpoint_bound(a, b, c, -1074, units);
        assert_midpoint_bound(
            F64::from_f64(a),
            F64::from_f64(b),
            F64::from_f64(c),
            -1074,
            F64::from_f64(units),
        );
    }
}

fn check_away_from_midpoint<T: RealField>(left_exponent: i32, right_exponent: i32, unit: T) {
    let left = T::ONE.scale_binary(left_exponent);
    let right = T::ONE.scale_binary(right_exponent);
    let margin = T::from_f64(8.0) * T::EPSILON;
    assert_eq!(squared_correlation(T::ONE - margin, left, right), T::ZERO);
    assert_eq!(squared_correlation(T::ONE + margin, left, right), unit);
}

#[test]
fn zero_classification_is_strict_away_from_rounding_midpoint() {
    check_away_from_midpoint::<f32>(75, 75, f32::from_bits(1));
    check_away_from_midpoint::<F32>(75, 75, F32::from_f32(f32::from_bits(1)));
    check_away_from_midpoint::<f64>(537, 538, f64::from_bits(1));
    check_away_from_midpoint::<F64>(537, 538, F64::from_f64(f64::from_bits(1)));
}
