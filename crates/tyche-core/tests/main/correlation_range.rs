//! Squared Pearson finalization when finite centered sums leave the product range.
//!
//! For observations `(-s, -s)` and `(s, s)` the centered sums satisfy
//! `A = B = C = 2s^2`, all exact powers of two, so the index `C^2/(AB)` is
//! exactly one. At `s = 2^40` (binary32) or `s = 2^300` (binary64) the stored
//! moments are finite while `C^2` and `AB` overflow; at the reciprocal scales
//! they underflow to zero. Integer designs with exact stored sums cover the
//! gradual-underflow regime, where the products are nonzero but subnormal, and
//! indices far below the ordinary range.

use eunomia::{F32, F64, RealField};
use tyche_core::CorrelationScreening;

/// Scales and integer-design parameters that place the test moments in a
/// precision's overflow, gradual-underflow and tiny-index regimes.
trait RangeScale: RealField {
    /// Power-of-two scale at which `A*B` overflows and `A*B` of the reciprocal
    /// scale underflows, while `A = 2s^2` itself stays finite and normal.
    const EXTREME_EXPONENT: i32;
    /// Scale `2^e` at which `C^2` and `AB` of the integer designs below lie in
    /// `[2^36 s^4, 2^43 s^4]`, strictly between the least subnormal and the
    /// smallest normal value, while `A` and `B` stay normal.
    const SUBNORMAL_PRODUCT_EXPONENT: i32;
    /// Exponent `emin - p + 1` of the least positive subnormal value.
    const LEAST_SUBNORMAL_EXPONENT: i32;
    /// Triples `(t, m, q)` selecting the tiny-index design of scale `2^t` and
    /// response perturbation `m*2^-q`. At scale one: an index in the normal
    /// range and a subnormal one. At a smaller scale: `C^2` a nonzero
    /// subnormal with `AB >= EPSILON` and a normal index (numerator-only
    /// gradual underflow).
    const TINY_DESIGNS: [(i32, i64, i32); 3];
}

impl RangeScale for f32 {
    const EXTREME_EXPONENT: i32 = 40;
    const SUBNORMAL_PRODUCT_EXPONENT: i32 = -45;
    const LEAST_SUBNORMAL_EXPONENT: i32 = -149;
    const TINY_DESIGNS: [(i32, i64, i32); 3] = [(0, 7, 60), (0, 5, 77), (-6, 7, 66)];
}
impl RangeScale for f64 {
    const EXTREME_EXPONENT: i32 = 300;
    const SUBNORMAL_PRODUCT_EXPONENT: i32 = -275;
    const LEAST_SUBNORMAL_EXPONENT: i32 = -1074;
    const TINY_DESIGNS: [(i32, i64, i32); 3] = [(0, 7, 200), (0, 7, 540), (-13, 5, 514)];
}
impl RangeScale for F32 {
    const EXTREME_EXPONENT: i32 = 40;
    const SUBNORMAL_PRODUCT_EXPONENT: i32 = -45;
    const LEAST_SUBNORMAL_EXPONENT: i32 = -149;
    const TINY_DESIGNS: [(i32, i64, i32); 3] = [(0, 7, 60), (0, 5, 77), (-6, 7, 66)];
}
impl RangeScale for F64 {
    const EXTREME_EXPONENT: i32 = 300;
    const SUBNORMAL_PRODUCT_EXPONENT: i32 = -275;
    const LEAST_SUBNORMAL_EXPONENT: i32 = -1074;
    const TINY_DESIGNS: [(i32, i64, i32); 3] = [(0, 7, 200), (0, 7, 540), (-13, 5, 514)];
}

fn power_of_two<T: RealField>(exponent: i32) -> T {
    T::ONE.scale_binary(exponent)
}

/// `gamma_k = ku/(1-ku)` for unit roundoff `u = EPSILON/2`: the relative
/// bound on `k` correctly rounded operations (Higham, Accuracy and Stability
/// of Numerical Algorithms, Lemma 3.1).
fn gamma<T: RealField>(roundings: i64) -> T {
    let bound = T::from_integer(roundings) * (T::EPSILON / (T::ONE + T::ONE));
    bound / (T::ONE - bound)
}

fn single_index<T: RealField>(screening: CorrelationScreening<T, 1>) -> T {
    screening
        .report()
        .expect("at least two observations")
        .squared_correlations()[0]
}

fn two_point_index<T: RealField>(parameter_sign: T, scale: T) -> T {
    let mut screening = CorrelationScreening::<T, 1>::new();
    screening.update(&[-parameter_sign * scale], -scale);
    screening.update(&[parameter_sign * scale], scale);
    single_index(screening)
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

/// Orthogonal-design observations of two parameters and two outputs.
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

/// Scaling every coordinate by a power of two scales each moment by an exact
/// power of two while all moments stay normal, so the unit-scale and scaled
/// estimators accumulate mantissas bit for bit. The ordinary and normalized
/// paths then round the same mantissa products, and the indices are equal.
fn scaled_design_equals_unit_design<T: RangeScale>() {
    let reference = design_report(T::ONE);
    for exponent in [-T::EXTREME_EXPONENT, T::EXTREME_EXPONENT] {
        let scaled = design_report(power_of_two::<T>(exponent));
        assert_eq!(scaled, reference, "scale 2^{exponent}");
    }
}

#[test]
fn design_indices_are_invariant_under_power_of_two_scaling() {
    scaled_design_equals_unit_design::<f32>();
    scaled_design_equals_unit_design::<f64>();
    scaled_design_equals_unit_design::<F32>();
    scaled_design_equals_unit_design::<F64>();
}

/// Greatest common divisor, reducing an exact index to a small fraction.
fn gcd(left: i64, right: i64) -> i64 {
    if right == 0 {
        left
    } else {
        gcd(right, left % right)
    }
}

const PARAMETER: i64 = 1009;
const UNIT: i64 = 297;
/// Zero-sum response shapes, multiplied by [`UNIT`].
const RESPONSE_SHAPES: [[i64; 3]; 3] = [[1, 0, -1], [2, -1, -1], [3, 1, -4]];

/// Nonzero subnormal `C^2` and `AB` with inexact products keep the exact index.
///
/// Parameters `(a, -a, 0)s` and zero-sum responses `rs` have exact running
/// means, so the stored sums are exactly `A = sum(a_i^2) s^2`,
/// `B = sum(r_i^2) s^2` and `C = sum(a_i r_i) s^2` and the exact index is the
/// rational `(sum a_i r_i)^2 / (sum a_i^2 sum r_i^2)`, here `1/4`, `3/4` and
/// `1/13`. Its numerator and denominator reduce to integers below `2^24`, so
/// the expected value `e = fl(n/d)` carries one rounding. With
/// `|r - x| <= gamma_3 x` and `|x - e| <= u x` for the exact index `x`, the
/// result `r` satisfies `|r - e| <= (gamma_3 + u)/(1 - u) e <= gamma_4 e`,
/// the bound asserted. The leading bit of each rounded subnormal product lies
/// 5 to 11 places (binary32) or 10 to 16 places (binary64) above the least
/// unit, so rounding costs a relative error up to `2^-6` or `2^-11` (half a
/// unit over `2^5` or `2^10` units), far above that bound.
fn subnormal_products_keep_exact_index<T: RangeScale>() {
    let scale = power_of_two::<T>(T::SUBNORMAL_PRODUCT_EXPONENT);
    let parameters = [PARAMETER, -PARAMETER, 0];
    let dot = |left: &[i64; 3], right: &[i64; 3]| -> i64 {
        left.iter().zip(right).map(|(l, r)| l * r).sum()
    };
    for shape in RESPONSE_SHAPES {
        let responses = shape.map(|r| r * UNIT);
        let mut screening = CorrelationScreening::<T, 1>::new();
        for (&parameter, &response) in parameters.iter().zip(&responses) {
            let observed = [T::from_integer(parameter) * scale];
            screening.update(&observed, T::from_integer(response) * scale);
        }
        let numerator = dot(&parameters, &responses).pow(2);
        let denominator = dot(&parameters, &parameters) * dot(&responses, &responses);
        let divisor = gcd(numerator, denominator);
        let expected =
            T::from_integer(numerator / divisor) / T::from_integer(denominator / divisor);
        let index = single_index(screening);
        assert!(
            (index - expected).abs() <= gamma::<T>(4) * expected,
            "{responses:?}: {index:?} vs {expected:?}"
        );
    }
}

#[test]
fn subnormal_products_report_the_exact_rational_index() {
    subnormal_products_keep_exact_index::<f32>();
    subnormal_products_keep_exact_index::<f64>();
    subnormal_products_keep_exact_index::<F32>();
    subnormal_products_keep_exact_index::<F64>();
}

/// A near-orthogonal design keeps a tiny nonzero index, down to the subnormals.
///
/// Parameters `(0, 0, 3t)` and responses `(t, -t, 3et)` have exact running
/// means, so `A = 6t^2` and `C = 6et^2` exactly, and `B = 2t^2 + 6e^2t^2`
/// rounds to `2t^2` because `6e^2t^2` lies below half an ulp of `2t^2`. The
/// stored sums give the exact index `(6et^2)^2/(6t^2 2t^2) = 3e^2`, for every
/// `t`, with `e = m 2^-q`. Scaling the report by `2^2q` makes the exact value
/// the integer `3m^2`, with a bound of `gamma_3` relative plus half a least
/// subnormal unit (`2^(L-1)`, scaled likewise): a subnormal index admits only
/// the nearest multiple of the least unit, and the second term is vacuous for
/// a normal one. At `t < 1` the product `C^2 = 36e^2t^4` is `13.8` (binary32)
/// or `14.1` (binary64) least units, so its leading bit sits 3 places up and
/// an ordinary quotient of that rounded product errs by up to `2^-4`, far
/// above `gamma_3`.
fn tiny_index_keeps_exact_scale<T: RangeScale>() {
    for (scale_exponent, mantissa, exponent) in T::TINY_DESIGNS {
        let scale = power_of_two::<T>(scale_exponent);
        let three = T::from_integer(3);
        let perturbation = T::from_integer(mantissa) * power_of_two::<T>(-exponent);
        let mut screening = CorrelationScreening::<T, 1>::new();
        for (parameter, response) in [
            (T::ZERO, scale),
            (T::ZERO, -scale),
            (three * scale, three * perturbation * scale),
        ] {
            screening.update(&[parameter], response);
        }
        let scaled = single_index(screening).scale_binary(2 * exponent);
        let exact = T::from_integer(3 * mantissa * mantissa);
        let half_unit = power_of_two::<T>(T::LEAST_SUBNORMAL_EXPONENT - 1 + 2 * exponent);
        // `exact` is the stored-sum index `3e^2` scaled by `2^2q`; the bound
        // is the one derived above.
        assert!(
            (scaled - exact).abs() <= gamma::<T>(3) * exact + half_unit,
            "e = {mantissa}*2^-{exponent}: {scaled:?} vs {exact:?}"
        );
    }
}

#[test]
fn tiny_indices_keep_their_exact_scale_down_to_subnormals() {
    tiny_index_keeps_exact_scale::<f32>();
    tiny_index_keeps_exact_scale::<f64>();
    tiny_index_keeps_exact_scale::<F32>();
    tiny_index_keeps_exact_scale::<F64>();
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
        assert_eq!(single_index(orthogonal), T::ZERO, "orthogonal 2^{exponent}");

        let mut constant = CorrelationScreening::<T, 1>::new();
        for response in [-scale, scale] {
            constant.update(&[scale], response);
        }
        assert_eq!(single_index(constant), T::ZERO, "constant 2^{exponent}");
    }
}

#[test]
fn degenerate_moments_report_zero_at_scale_extremes() {
    degenerate_moments_report_zero::<f32>();
    degenerate_moments_report_zero::<f64>();
    degenerate_moments_report_zero::<F32>();
    degenerate_moments_report_zero::<F64>();
}
