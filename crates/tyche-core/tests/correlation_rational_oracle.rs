//! Differential Pearson laws from exact integer centered sums.

use eunomia::{F32, F64, RealField};
use proptest::prelude::*;
use tyche_core::CorrelationScreening;

fn exact_index(x: &[i16; 8], y: &[i16; 8]) -> f64 {
    let sum_x: i64 = x.iter().map(|&value| i64::from(value)).sum();
    let sum_y: i64 = y.iter().map(|&value| i64::from(value)).sum();
    let squares =
        |values: &[i16; 8]| -> i64 { values.iter().map(|&value| i64::from(value).pow(2)).sum() };
    let cross: i64 = x
        .iter()
        .zip(y)
        .map(|(&left, &right)| i64::from(left) * i64::from(right))
        .sum();
    let left = 8 * squares(x) - sum_x * sum_x;
    let right = 8 * squares(y) - sum_y * sum_y;
    let covariance = 8 * cross - sum_x * sum_y;
    if left == 0 || right == 0 {
        return 0.0;
    }
    // Each centered integer sum is at most 16,384, so these exact integer
    // products fit u32 and their conversion to f64 loses no bits. The sole
    // reference rounding is the final rational division.
    f64::from(u32::try_from(covariance * covariance).expect("bounded square"))
        / f64::from(u32::try_from(left * right).expect("bounded product"))
}

fn compare<T: RealField>(x: &[i16; 8], y: &[i16; 8], exponents: (i32, i32)) {
    let mut screening = CorrelationScreening::<T, 1>::new();
    for (&left, &right) in x.iter().zip(y) {
        screening.update(
            &[T::from_f64(f64::from(left)).scale_binary(exponents.0)],
            T::from_f64(f64::from(right)).scale_binary(exponents.1),
        );
    }
    let actual = screening
        .report()
        .expect("eight observations")
        .squared_correlations()[0];
    let expected = T::from_f64(exact_index(x, y));
    assert!(
        (actual - expected).abs() <= T::from_f64(64.0) * T::EPSILON,
        "scales {exponents:?}: {actual:?} != {expected:?}"
    );
}

proptest! {
    #[test]
    fn scale_invariance_against_integer_rational_oracle(
        x in prop::array::uniform8(-16_i16..=16),
        y in prop::array::uniform8(-16_i16..=16),
        small_exponents in (-60_i32..=55, -60_i32..=55),
        wide_exponents in (-500_i32..=490, -500_i32..=490),
    ) {
        compare::<f32>(&x, &y, small_exponents);
        compare::<F32>(&x, &y, small_exponents);
        compare::<f64>(&x, &y, wide_exponents);
        compare::<F64>(&x, &y, wide_exponents);
    }
}
