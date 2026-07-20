//! Statistics and sensitivity evidence.

use tyche_core::{CorrelationScreening, Moments, PopulationVariance, SampleVariance};

#[test]
fn welford_matches_two_pass_oracle() {
    let values = [1.0_f64, 2.0, 4.0, 8.0, 16.0, 32.0];
    let mean = values.iter().sum::<f64>() / 6.0;
    let population = values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / 6.0;
    let mut moments = Moments::new();
    for value in values {
        moments.update(value);
    }
    assert_eq!(moments.mean().expect("defined").to_bits(), mean.to_bits());
    assert_eq!(
        moments
            .variance::<PopulationVariance>()
            .expect("defined")
            .to_bits(),
        population.to_bits()
    );
    assert_eq!(
        moments
            .variance::<SampleVariance>()
            .expect("defined")
            .to_bits(),
        (population * 6.0 / 5.0).to_bits()
    );
}

#[test]
fn chan_merge_and_singleton_policy_are_explicit() {
    let mut left = Moments::new();
    let mut right = Moments::new();
    for value in [1.0_f64, 2.0, 3.0] {
        left.update(value);
    }
    for value in [4.0_f64, 5.0, 6.0] {
        right.update(value);
    }
    left.merge(right);
    assert_eq!(left.mean().expect("defined").to_bits(), 3.5_f64.to_bits());
    assert_eq!(left.centered_sum().to_bits(), 17.5_f64.to_bits());
    let mut singleton = Moments::<f64>::new();
    singleton.update(3.0);
    assert_eq!(
        singleton
            .variance::<PopulationVariance>()
            .expect("defined")
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        singleton
            .variance::<SampleVariance>()
            .expect_err("undefined")
            .required(),
        2
    );
}

#[test]
fn affine_one_parameter_response_has_unit_index() {
    let mut screening = CorrelationScreening::<f64, 1>::new();
    for index in -50..=50 {
        let x = f64::from(index) / 10.0;
        screening.update(&[x], 3.0 * x - 7.0);
    }
    let value = screening.report().expect("defined").squared_correlations()[0];
    assert!((value - 1.0).abs() <= 8.0 * f64::EPSILON);
}
