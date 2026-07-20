//! Executable evidence for online statistics and sensitivity.

use tyche_core::{CorrelationScreening, Moments, PopulationVariance, SampleVariance};

fn direct_population(values: &[f64]) -> (f64, f64) {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64;
    (mean, variance)
}

#[test]
fn welford_matches_independent_two_pass_oracle() {
    let values = [1.0_f64, 2.0, 4.0, 8.0, 16.0, 32.0];
    let mut moments = Moments::new();
    for value in values {
        moments.update(value);
    }
    let (mean, population) = direct_population(&values);
    assert_eq!(moments.mean().expect("non-empty"), mean);
    assert_eq!(
        moments.variance::<PopulationVariance>().expect("non-empty"),
        population
    );
    assert_eq!(
        moments
            .variance::<SampleVariance>()
            .expect("multiple values"),
        population * values.len() as f64 / (values.len() - 1) as f64
    );
}

#[test]
fn chan_merge_preserves_moments_in_logical_order() {
    let mut left = Moments::new();
    let mut right = Moments::new();
    for value in [1.0_f64, 2.0, 3.0] {
        left.update(value);
    }
    for value in [4.0_f64, 5.0, 6.0] {
        right.update(value);
    }
    left.merge(right);

    assert_eq!(left.count(), 6);
    assert_eq!(left.mean().expect("non-empty"), 3.5);
    assert_eq!(left.centered_sum(), 17.5);
}

#[test]
fn singleton_sample_variance_is_typed_failure_not_nan() {
    let mut moments = Moments::new();
    moments.update(3.0_f64);

    assert_eq!(
        moments
            .variance::<PopulationVariance>()
            .expect("population variance exists"),
        0.0
    );
    let error = moments
        .variance::<SampleVariance>()
        .expect_err("sample variance requires two observations");
    assert_eq!(error.required(), 2);
    assert_eq!(error.actual(), 1);
}

#[test]
fn affine_one_parameter_response_has_unit_screening_index() {
    let mut screening = CorrelationScreening::<f64, 1>::new();
    for index in -50..=50 {
        let parameter = f64::from(index) / 10.0;
        screening.update(&[parameter], 3.0 * parameter - 7.0);
    }
    let report = screening.report().expect("multiple nonconstant samples");
    assert!((report.squared_correlations()[0] - 1.0).abs() <= 8.0 * f64::EPSILON);
}
