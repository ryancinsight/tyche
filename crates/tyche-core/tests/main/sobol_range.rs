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
