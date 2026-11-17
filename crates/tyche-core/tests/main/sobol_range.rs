//! Analytical offset and population-variance laws for Saltelli screening.

use eunomia::RealField;
use mnemosyne::counting::{AllocationDelta, measure};
use tyche_core::SobolIndices;

fn check_offsets<T: RealField>(offset: T) {
    let zero = T::ZERO;
    let one = T::ONE;
    // A and B independently enumerate the balanced {-1, +1} response.
    // A's population variance is exactly 1, centered sum 4. Replacing the
    // only influential parameter gives B, so cross=4 and squared defect=8.
    // This ordering makes the Welford centered sum exact in both precisions.
    let a = [-one, one, -one, one];
    let b = [-one, -one, one, one];
    for shift in [zero, offset, -offset] {
        let mut scalar = SobolIndices::<T, 1>::new();
        let mut outputs = SobolIndices::<T, 2, 2>::new();
        for (&a, &b) in a.iter().zip(&b) {
            let base = shift + a;
            let independent = shift + b;
            scalar.update(base, independent, &[independent]);
            // Second parameter has no influence. Second output is constant.
            outputs.update_outputs(
                &[base, shift],
                &[independent, shift],
                &[[independent, base], [shift, shift]],
            );
        }
        let scalar = scalar.report().expect("four balanced rows");
        assert_eq!(scalar.sample_count(), 4);
        assert_eq!(scalar.first_order(), &[one]);
        assert_eq!(scalar.total_order(), &[one]);
        let outputs = outputs.report().expect("four balanced rows per output");
        assert_eq!(outputs.sample_count(), 4);
        assert_eq!(
            outputs.first_order_by_output(),
            &[[one, zero], [zero, zero]]
        );
        assert_eq!(
            outputs.total_order_by_output(),
            &[[one, zero], [zero, zero]]
        );
    }
}

#[test]
fn sobol_indices_preserve_influence_under_exact_response_offsets() {
    check_offsets(10_000.0_f32);
    check_offsets(100_000_000.0_f64);
}

#[test]
fn sobol_denominator_uses_population_not_sample_variance() {
    // Two rows have A=(-1,+1), hence N*V=2. The squared defect is 1,
    // so the total index is 1/4, not 1/8 from the unbiased variance.
    let mut indices = SobolIndices::<f64, 1>::new();
    indices.update(-1.0, 0.0, &[0.0]);
    indices.update(1.0, 1.0, &[1.0]);
    let report = indices.report().expect("two observations");
    assert_eq!(report.first_order()[0].to_bits(), 0.0_f64.to_bits());
    assert_eq!(report.total_order()[0].to_bits(), 0.25_f64.to_bits());
    assert_eq!(report.sample_count(), 2);
}

#[test]
fn centered_sobol_updates_and_reporting_allocate_nothing() {
    let (report, allocations) = measure(|| {
        let mut indices = SobolIndices::<f64, 1>::new();
        for (base, independent) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
            indices.update(base, independent, &[independent]);
        }
        indices.report().expect("four balanced observations")
    });
    assert_eq!(allocations, AllocationDelta::default());
    assert_eq!(report.first_order()[0].to_bits(), 1.0_f64.to_bits());
    assert_eq!(report.total_order()[0].to_bits(), 1.0_f64.to_bits());
    assert_eq!(report.sample_count(), 4);
}

fn two_row_total<T: RealField>(d: T, b: T) -> T {
    let mut indices = SobolIndices::<T, 1>::new();
    indices.update(T::ZERO, d, &[d]);
    indices.update(b, b, &[b]);
    indices.report().expect("two observations").total_order()[0]
}

#[test]
fn total_order_preserves_minimum_subnormal_f32() {
    // The independent exact-binary oracle rounds d²/b² to the least
    // subnormal. Rounding d²/M2 before halving instead produces zero.
    assert_eq!(
        two_row_total(f32::from_bits(0x1a40_0000), f32::from_bits(0x3fb5_04f2)).to_bits(),
        1
    );
    // Overflowed 2*M2 needs the same one-rounding behavior. Here b=2^64,
    // d=9*2^-14, so d²/b²=(81/128)*2^-149 rounds to 2^-149.
    assert_eq!(
        two_row_total(9.0_f32 * 2.0_f32.powi(-14), 2.0_f32.powi(64)).to_bits(),
        1
    );
}

#[test]
fn total_order_preserves_minimum_subnormal_f64() {
    assert_eq!(
        two_row_total(
            f64::from_bits(0x1e60_0000_0000_0000),
            f64::from_bits(0x3ff6_a09e_667f_3bcc),
        )
        .to_bits(),
        1
    );
    // b=2^512 and d=13*2^-29 give d²/b²=(169/256)*2^-1074.
    assert_eq!(
        two_row_total(13.0_f64 * 2.0_f64.powi(-29), 2.0_f64.powi(512)).to_bits(),
        1
    );
}

fn check_overflowed_total_denominator<T: RealField>(exponent: i32) {
    let two = T::from_f64(2.0);
    let b = T::ONE.scale_binary(exponent);
    let centered_sum = b * (b / two);
    assert!(centered_sum.is_finite());
    assert!(!(two * centered_sum).is_finite());
    // A=(0,b), B=(b/2,b), so M2=b²/2, squared defect=b²/4,
    // and total-order influence=(b²/4)/b²=1/4 exactly.
    assert_eq!(two_row_total(b / two, b), T::from_f64(0.25));
}

#[test]
fn total_order_does_not_overflow_a_finite_centered_sum() {
    check_overflowed_total_denominator::<f32>(64);
    check_overflowed_total_denominator::<f64>(512);
}
