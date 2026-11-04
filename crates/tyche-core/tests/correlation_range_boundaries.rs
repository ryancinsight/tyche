//! Public correlation laws across independent coordinate scales.

use eunomia::{F32, F64, FloatElement, RealField};
use mnemosyne::counting::{AllocationDelta, measure};
use tyche_core::CorrelationScreening;

fn check_scale_grid<T: RealField>(exponents: &[i32]) {
    for &parameter_exponent in exponents {
        for &response_exponent in exponents {
            let x_scale = T::ONE.scale_binary(parameter_exponent);
            let y_scale = T::ONE.scale_binary(response_exponent);
            let mut screening = CorrelationScreening::<T, 3, 2>::new();
            // Orthogonal balanced sign columns have equal variance. The two
            // response laws are x0 + 2*x1 and 3*x0 - x1, so the squared
            // correlations are (1/5, 4/5) and (9/10, 1/10).
            for (x0, x1) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let x0 = T::from_f64(f64::from(x0));
                let x1 = T::from_f64(f64::from(x1));
                screening.update_outputs(
                    &[x0 * x_scale, x1 * x_scale, T::ONE],
                    &[
                        (x0 + T::from_f64(2.0) * x1) * y_scale,
                        (T::from_f64(3.0) * x0 - x1) * y_scale,
                    ],
                );
            }
            let report = screening.report().expect("four observations");
            assert_eq!(report.sample_count(), 4);
            for (actual, expected) in report
                .squared_correlations_by_output()
                .iter()
                .flatten()
                .zip([0.2, 0.8, 0.0, 0.9, 0.1, 0.0])
            {
                // Four Welford updates and the final quotient contribute a
                // bounded number of roundings; these well-conditioned laws
                // have margin within 32 native epsilons, independent of scale.
                assert!(
                    (*actual - T::from_f64(expected)).abs() <= T::from_f64(32.0) * T::EPSILON,
                    "scales ({parameter_exponent}, {response_exponent}), {actual:?} != {expected}"
                );
            }
        }
    }
}

#[test]
fn independent_scale_and_output_matrix() {
    check_scale_grid::<f32>(&[-60, -40, -20, 0, 20, 40, 60]);
    check_scale_grid::<F32>(&[-60, -40, -20, 0, 20, 40, 60]);
    check_scale_grid::<f64>(&[-500, -300, -150, 0, 150, 300, 500]);
    check_scale_grid::<F64>(&[-500, -300, -150, 0, 150, 300, 500]);
}

fn check_small_moments<T: RealField>(exponent: i32) {
    let scale = T::ONE.scale_binary(exponent);
    let mut screening = CorrelationScreening::<T, 1, 2>::new();
    screening.update_outputs(&[-scale], &[scale, T::ZERO]);
    screening.update_outputs(&[scale], &[-scale, T::ZERO]);
    assert_eq!(
        *screening
            .report()
            .expect("two observations")
            .squared_correlations_by_output(),
        [[T::ONE], [T::ZERO]]
    );
}

#[test]
fn subnormal_moments_keep_negative_perfect_correlation() {
    check_small_moments::<f32>(-74);
    check_small_moments::<F32>(-74);
    check_small_moments::<f64>(-537);
    check_small_moments::<F64>(-537);
}

fn small_index<T: RealField>(exponent: i32) -> T {
    let small = T::ONE.scale_binary(exponent);
    let mut screening = CorrelationScreening::<T, 1>::new();
    for (parameter, response) in [
        (T::ONE, small),
        (-T::ONE, -small),
        (T::ZERO, T::ONE),
        (T::ZERO, -T::ONE),
    ] {
        screening.update(&[parameter], response);
    }
    screening
        .report()
        .expect("four observations")
        .squared_correlations()[0]
}

#[test]
fn representable_subnormal_indices_and_true_underflow() {
    assert_eq!(small_index::<f32>(-74).to_bits(), 2);
    assert_eq!(small_index::<F32>(-74), F32::from_f32(f32::from_bits(2)));
    assert_eq!(small_index::<f64>(-537).to_bits(), 1);
    assert_eq!(small_index::<F64>(-537), F64::from_f64(f64::from_bits(1)));
    assert_eq!(small_index::<f32>(-76).to_bits(), 0);
    assert_eq!(small_index::<f64>(-539).to_bits(), 0);
}

#[test]
fn insufficient_observations_retain_typed_error() {
    let mut screening = CorrelationScreening::<f64, 2, 2>::new();
    assert_eq!(screening.report().expect_err("empty").required(), 2);
    screening.update_outputs(&[1.0, 2.0], &[3.0, 4.0]);
    assert_eq!(screening.report().expect_err("singleton").required(), 2);
}

#[test]
fn construction_updates_and_report_allocate_nothing() {
    let ((), allocations) = measure(|| {
        check_scale_grid::<f32>(&[-40, 0, 40]);
        check_scale_grid::<F32>(&[-40, 0, 40]);
        check_scale_grid::<f64>(&[-300, 0, 300]);
        check_scale_grid::<F64>(&[-300, 0, 300]);
    });
    assert_eq!(allocations, AllocationDelta::default());
}
