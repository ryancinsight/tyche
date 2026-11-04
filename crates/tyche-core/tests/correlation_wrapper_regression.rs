//! Wrapper regressions execute independently of the primitive assertions.

use eunomia::{F32, F64, RealField};
use tyche_core::CorrelationScreening;

fn check<T: RealField>(exponent: i32) {
    let scale = T::ONE.scale_binary(exponent);
    let mut screening = CorrelationScreening::<T, 1>::new();
    screening.update(&[-scale], -scale);
    screening.update(&[scale], scale);
    assert_eq!(
        screening
            .report()
            .expect("two samples")
            .squared_correlations()[0],
        T::ONE
    );
}

#[test]
fn wrapper_binary32_product_overflow() {
    check::<F32>(40);
}
#[test]
fn wrapper_binary32_product_underflow() {
    check::<F32>(-40);
}
#[test]
fn wrapper_binary64_product_overflow() {
    check::<F64>(300);
}
#[test]
fn wrapper_binary64_product_underflow() {
    check::<F64>(-300);
}
