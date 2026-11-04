//! Pearson correlation is invariant under nonzero changes of coordinate scale.

use eunomia::{F32, F64, FloatElement, RealField};
use tyche_core::CorrelationScreening;

fn identical_coordinates_have_unit_index<T: RealField>(scale: T) {
    let mut screening = CorrelationScreening::<T, 1>::new();
    screening.update(&[-scale], -scale);
    screening.update(&[scale], scale);
    let report = screening.report().expect("two observations");
    let actual = report.squared_correlations()[0];
    assert_eq!(report.sample_count(), 2);
    assert_eq!(actual, T::ONE, "nonconstant identical coordinates");
}

#[test]
fn binary32_finite_moments_survive_product_overflow() {
    identical_coordinates_have_unit_index::<f32>(2.0_f32.powi(40));
    identical_coordinates_have_unit_index::<F32>(F32::from_f64(2.0_f64.powi(40)));
}

#[test]
fn binary64_finite_moments_survive_product_overflow() {
    identical_coordinates_have_unit_index::<f64>(2.0_f64.powi(300));
    identical_coordinates_have_unit_index::<F64>(F64::from_f64(2.0_f64.powi(300)));
}

#[test]
fn binary32_finite_moments_survive_product_underflow() {
    identical_coordinates_have_unit_index::<f32>(2.0_f32.powi(-40));
    identical_coordinates_have_unit_index::<F32>(F32::from_f64(2.0_f64.powi(-40)));
}

#[test]
fn binary64_finite_moments_survive_product_underflow() {
    identical_coordinates_have_unit_index::<f64>(2.0_f64.powi(-300));
    identical_coordinates_have_unit_index::<F64>(F64::from_f64(2.0_f64.powi(-300)));
}

#[test]
fn ordinary_coordinates_control() {
    identical_coordinates_have_unit_index::<f32>(2.0);
    identical_coordinates_have_unit_index::<f64>(2.0);
    identical_coordinates_have_unit_index::<F32>(F32::from_f64(2.0));
    identical_coordinates_have_unit_index::<F64>(F64::from_f64(2.0));
}
