//! Squared Pearson finalization when finite centered sums leave the product range.
//!
//! For observations `(-s, -s)` and `(s, s)` the centered sums satisfy
//! `A = B = C = 2s^2`, all exact powers of two, so the index `C^2/(AB)` is
//! exactly one. At `s = 2^40` (binary32) or `s = 2^300` (binary64) the stored
//! moments are finite while `C^2` and `AB` overflow; at the reciprocal scales
//! they underflow to zero.

use eunomia::{F32, F64, RealField};
use tyche_core::CorrelationScreening;

/// Power-of-two scale at which `A*B` overflows and `A*B` of the reciprocal
/// scale underflows, while `A = 2s^2` itself stays finite and normal.
trait RangeScale: RealField {
    const EXTREME_EXPONENT: i32;
}

impl RangeScale for f32 {
    const EXTREME_EXPONENT: i32 = 40;
}
impl RangeScale for f64 {
    const EXTREME_EXPONENT: i32 = 300;
}
impl RangeScale for F32 {
    const EXTREME_EXPONENT: i32 = 40;
}
impl RangeScale for F64 {
    const EXTREME_EXPONENT: i32 = 300;
}

fn power_of_two<T: RealField>(exponent: i32) -> T {
    T::ONE.scale_binary(exponent)
}

fn two_point_index<T: RealField>(parameter_sign: T, scale: T) -> T {
    let mut screening = CorrelationScreening::<T, 1>::new();
    screening.update(&[-parameter_sign * scale], -scale);
    screening.update(&[parameter_sign * scale], scale);
    screening
        .report()
        .expect("two observations")
        .squared_correlations()[0]
}

fn unit_index_at_scale_extremes<T: RangeScale>() {
    for exponent in [-T::EXTREME_EXPONENT, 0, T::EXTREME_EXPONENT] {
        let scale = power_of_two::<T>(exponent);
        for parameter_sign in [T::ONE, -T::ONE] {
            let index = two_point_index(parameter_sign, scale);
            assert_eq!(index, T::ONE, "scale 2^{exponent}");
        }
    }
}

#[test]
fn identical_coordinates_have_unit_index_at_scale_extremes() {
    unit_index_at_scale_extremes::<f32>();
    unit_index_at_scale_extremes::<f64>();
    unit_index_at_scale_extremes::<F32>();
    unit_index_at_scale_extremes::<F64>();
}

/// Orthogonal-design observations of two parameters and two outputs whose
/// exact squared indices are 1/5, 4/5, 9/10 and 1/10.
fn design_report<T: RealField>(scale: T) -> [[T; 2]; 2] {
    let mut screening = CorrelationScreening::<T, 2, 2>::new();
    for x0 in [-T::ONE, T::ZERO, T::ONE] {
        for x1 in [-T::ONE, T::ZERO, T::ONE] {
            let two = T::ONE + T::ONE;
            let three = two + T::ONE;
            screening.update_outputs(
                &[x0 * scale, x1 * scale],
                &[(x0 + two * x1) * scale, (three * x0 - x1) * scale],
            );
        }
    }
    *screening
        .report()
        .expect("nine observations")
        .squared_correlations_by_output()
}

/// Scaling every coordinate by a power of two leaves each moment ratio
/// unchanged, and scaling is exact, so the unit-scale and scaled estimators
/// accumulate bit-identical mantissas. The two finalization paths each round
/// three times (`C^2`, `AB`, quotient) with relative forward error at most
/// `gamma_3 = 3u/(1-3u)` for unit roundoff `u = EPSILON/2`, so the paths differ
/// by at most `2*gamma_3` times the index.
fn scaled_design_matches_unit_design<T: RangeScale>() {
    let unit_roundoff = T::EPSILON / (T::ONE + T::ONE);
    let three_u = (T::ONE + T::ONE + T::ONE) * unit_roundoff;
    let gamma_3 = three_u / (T::ONE - three_u);
    let reference = design_report(T::ONE);
    for exponent in [-T::EXTREME_EXPONENT, T::EXTREME_EXPONENT] {
        let scaled = design_report(power_of_two::<T>(exponent));
        for (scaled_row, reference_row) in scaled.iter().zip(reference.iter()) {
            for (&actual, &expected) in scaled_row.iter().zip(reference_row.iter()) {
                let bound = (T::ONE + T::ONE) * gamma_3 * expected;
                assert!(
                    (actual - expected).abs() <= bound,
                    "scale 2^{exponent}: {actual:?} vs {expected:?}"
                );
            }
        }
    }
}

#[test]
fn design_indices_are_invariant_under_power_of_two_scaling() {
    scaled_design_matches_unit_design::<f32>();
    scaled_design_matches_unit_design::<f64>();
    scaled_design_matches_unit_design::<F32>();
    scaled_design_matches_unit_design::<F64>();
}

/// Zero covariance with finite centered sums, and a constant coordinate,
/// report zero rather than a non-finite quotient.
///
/// Parameters `(-s, -s, 2s)` and responses `(s, -s, 0)` have exact means
/// (`3s/3` is exact), `A = 6s^2`, `B = 2s^2` and every covariance term has a
/// zero factor, so `C = 0` exactly. At the reciprocal scale `AB` underflows to
/// zero (`0/0`); at the large scale it overflows (`0/inf`).
fn degenerate_moments_report_zero<T: RangeScale>() {
    let two = T::ONE + T::ONE;
    for exponent in [-T::EXTREME_EXPONENT, T::EXTREME_EXPONENT] {
        let scale = power_of_two::<T>(exponent);
        let mut orthogonal = CorrelationScreening::<T, 1>::new();
        for (parameter, response) in [(-scale, scale), (-scale, -scale), (two * scale, T::ZERO)] {
            orthogonal.update(&[parameter], response);
        }
        let report = orthogonal.report().expect("three observations");
        assert_eq!(
            report.squared_correlations()[0],
            T::ZERO,
            "orthogonal 2^{exponent}"
        );

        let mut constant = CorrelationScreening::<T, 1>::new();
        for response in [-scale, scale] {
            constant.update(&[scale], response);
        }
        let report = constant.report().expect("two observations");
        assert_eq!(
            report.squared_correlations()[0],
            T::ZERO,
            "constant 2^{exponent}"
        );
    }
}

#[test]
fn degenerate_moments_report_zero_at_scale_extremes() {
    degenerate_moments_report_zero::<f32>();
    degenerate_moments_report_zero::<f64>();
    degenerate_moments_report_zero::<F32>();
    degenerate_moments_report_zero::<F64>();
}
