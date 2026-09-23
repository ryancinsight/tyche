//! Statistics and sensitivity evidence.

use tyche_core::{
    CorrelationScreening, Counter, ElementaryEffects, ElementaryEffectsError, Moments,
    MorrisScreening, PopulationVariance, SampleVariance, Seed, SobolIndices, SplitMix64,
    UserDomain,
};

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

#[test]
fn elementary_effects_reduce_a_linear_trajectory_exactly() {
    // y = x0 + x1, start (0, 0), perturb x1 then x0 by delta = 0.25.
    let effects = ElementaryEffects::<f64, 2>::from_steps(&[1, 0], 0.0, &[0.25, 0.5], 0.25)
        .expect("valid trajectory");
    assert_eq!(
        (*effects.effects()).map(f64::to_bits),
        [1.0_f64.to_bits(); 2]
    );
}

#[test]
fn elementary_effects_reject_invalid_trajectories() {
    assert!(matches!(
        ElementaryEffects::<f64, 2>::from_steps(&[0, 0], 0.0, &[0.5, 1.0], 0.5),
        Err(ElementaryEffectsError::DuplicateParameter { parameter: 0 })
    ));
    assert!(matches!(
        ElementaryEffects::<f64, 2>::from_steps(&[0, 2], 0.0, &[0.5, 1.0], 0.5),
        Err(ElementaryEffectsError::OutOfRangeParameter { step: 1, index: 2 })
    ));
    assert!(matches!(
        ElementaryEffects::<f64, 2>::from_steps(&[1, 0], 0.0, &[0.5, 1.0], 0.0),
        Err(ElementaryEffectsError::NonPositiveStep)
    ));
}

#[test]
fn morris_linear_model_has_unit_effects_and_zero_sigma() {
    let effects = ElementaryEffects::<f64, 2>::from_steps(&[1, 0], 0.0, &[0.25, 0.5], 0.25)
        .expect("valid trajectory");
    let mut screening = MorrisScreening::<f64, 2>::new();
    screening.update(effects.effects());
    screening.update(effects.effects());
    let report = screening.report().expect("defined");
    assert_eq!(report.effect_count(), 2);
    assert_eq!((*report.mu()).map(f64::to_bits), [1.0_f64.to_bits(); 2]);
    assert_eq!(
        (*report.mu_star()).map(f64::to_bits),
        [1.0_f64.to_bits(); 2]
    );
    assert_eq!((*report.sigma()).map(f64::to_bits), [0.0_f64.to_bits(); 2]);
    assert_eq!(
        MorrisScreening::<f64, 2>::new()
            .report()
            .expect_err("undefined")
            .required(),
        2
    );
}

#[test]
fn saltelli_matches_two_pass_oracle() {
    let a = [0.1_f64, 0.5, 0.9, 0.3];
    let b = [0.6_f64, 0.2, 0.8, 0.4];
    // A_i^B = A with column i replaced from B, for two parameters.
    let recombined_0 = [0.6_f64, 0.5, 0.8, 0.3];
    let recombined_1 = [0.1_f64, 0.2, 0.9, 0.4];
    let mut estimator = SobolIndices::<f64, 2>::new();
    for (row, &base) in a.iter().enumerate() {
        estimator.update(base, b[row], &[recombined_0[row], recombined_1[row]]);
    }
    let report = estimator.report().expect("defined");
    let count = f64::from(u32::try_from(a.len()).expect("fits u32"));
    let mean = a.iter().sum::<f64>() / count;
    let variance = a.iter().map(|value| value * value).sum::<f64>() / count - mean * mean;
    let expected_first = |recombined: &[f64; 4]| {
        let cross = (0..a.len())
            .map(|row| b[row] * (recombined[row] - a[row]))
            .sum::<f64>();
        (cross / (count * variance)).clamp(0.0, 1.0)
    };
    let expected_total = |recombined: &[f64; 4]| {
        let squares = (0..a.len())
            .map(|row| (a[row] - recombined[row]).powi(2))
            .sum::<f64>();
        (squares / (2.0 * count * variance)).clamp(0.0, 1.0)
    };
    assert_eq!(
        report.first_order()[0].to_bits(),
        expected_first(&recombined_0).to_bits()
    );
    assert_eq!(
        report.first_order()[1].to_bits(),
        expected_first(&recombined_1).to_bits()
    );
    assert_eq!(
        report.total_order()[0].to_bits(),
        expected_total(&recombined_0).to_bits()
    );
    assert_eq!(
        report.total_order()[1].to_bits(),
        expected_total(&recombined_1).to_bits()
    );
    assert_eq!(report.sample_count(), 4);
    assert_eq!(
        SobolIndices::<f64, 2>::new()
            .report()
            .expect_err("undefined")
            .required(),
        2
    );
}

#[test]
fn saltelli_recovers_balanced_indices_on_seeded_independent_streams() {
    type BaseDomain = UserDomain<0x6261_7365>;
    type IndependentDomain = UserDomain<0x696e_6465>;
    const ROWS: u64 = 16_384;
    const PARAMETERS: usize = 2;
    let seed = Seed::new(0x5361_6c74_656c_6c69); // "Saltelli"
    // The A/B/A_i^B scheme requires A and B to be independent sample
    // matrices. Distinct stream domains over SplitMix64 provide reproducible,
    // effectively independent uniforms with no shared-sequence pairing; the
    // same-index row triples are then a genuine Monte Carlo Saltelli design.
    let base = |row: u64, dimension: u64| {
        Counter::<BaseDomain, SplitMix64>::open_unit::<f64>(seed, row, dimension)
    };
    let independent = |row: u64, dimension: u64| {
        Counter::<IndependentDomain, SplitMix64>::open_unit::<f64>(seed, row, dimension)
    };
    // f(x) = x0 + x1, so both parameters share half the output variance.
    let mut estimator = SobolIndices::<f64, PARAMETERS>::new();
    for row in 0..ROWS {
        let a0 = base(row, 0);
        let a1 = base(row, 1);
        let b0 = independent(row, 0);
        let b1 = independent(row, 1);
        let base_response = a0 + a1;
        let independent_response = b0 + b1;
        // A_i^B = A with column i taken from B.
        let recombined_0 = b0 + a1;
        let recombined_1 = a0 + b1;
        estimator.update(
            base_response,
            independent_response,
            &[recombined_0, recombined_1],
        );
    }
    let report = estimator.report().expect("defined");
    for parameter in 0..PARAMETERS {
        let first = report.first_order()[parameter];
        let total = report.total_order()[parameter];
        assert!(
            (first - 0.5).abs() <= 0.05,
            "S_{parameter} = {first} deviates from 0.5"
        );
        assert!(
            (total - 0.5).abs() <= 0.05,
            "S_T{parameter} = {total} deviates from 0.5"
        );
        assert!(total <= 1.0);
    }
}

#[test]
fn saltelli_constant_response_leaves_indices_at_zero() {
    // Degenerate A responses have zero variance, so the indices are
    // undefined; the report must stay at zero rather than emitting NaN.
    let mut estimator = SobolIndices::<f64, 2>::new();
    for _ in 0..8 {
        estimator.update(5.0, 3.0, &[5.0, 5.0]);
    }
    let report = estimator.report().expect("defined");
    assert_eq!(
        (*report.first_order()).map(f64::to_bits),
        [0.0_f64.to_bits(); 2]
    );
    assert_eq!(
        (*report.total_order()).map(f64::to_bits),
        [0.0_f64.to_bits(); 2]
    );
}

#[test]
fn multi_output_sensitivity_preserves_each_output_contract() {
    type MultiBaseDomain = UserDomain<0x006d_756c_7469_6261>;
    type MultiIndependentDomain = UserDomain<0x006d_756c_7469_6262>;
    let seed = Seed::new(0x5361_6c74_656c_6c69);
    let mut correlation = CorrelationScreening::<f64, 2, 2>::new();
    for x0 in [-1.0_f64, 0.0, 1.0] {
        for x1 in [-1.0_f64, 0.0, 1.0] {
            correlation.update_outputs(&[x0, x1], &[x0 + 2.0 * x1, 3.0 * x0 - x1]);
        }
    }
    let correlation_report = correlation.report().expect("defined");
    assert!((correlation_report.squared_correlations_by_output()[0][0] - 0.2).abs() <= 1e-12);
    assert!((correlation_report.squared_correlations_by_output()[0][1] - 0.8).abs() <= 1e-12);
    assert!((correlation_report.squared_correlations_by_output()[1][0] - 0.9).abs() <= 1e-12);
    assert!((correlation_report.squared_correlations_by_output()[1][1] - 0.1).abs() <= 1e-12);

    let effects = ElementaryEffects::<f64, 2, 2>::from_steps_outputs(
        &[1, 0],
        &[0.0, 0.0],
        &[[0.25, -0.75], [0.5, -0.25]],
        0.25,
    )
    .expect("valid multi-output trajectory");
    assert_eq!(
        effects.effects_by_output()[0].map(f64::to_bits),
        [1.0_f64.to_bits(), 1.0_f64.to_bits()]
    );
    assert_eq!(
        effects.effects_by_output()[1].map(f64::to_bits),
        [2.0_f64.to_bits(), (-3.0_f64).to_bits()]
    );
    let mut morris = MorrisScreening::<f64, 2, 2>::new();
    morris.update_outputs(effects.effects_by_output());
    morris.update_outputs(effects.effects_by_output());
    let morris_report = morris.report().expect("defined");
    assert_eq!(
        morris_report.mu_by_output()[0].map(f64::to_bits),
        [1.0_f64.to_bits(), 1.0_f64.to_bits()]
    );
    assert_eq!(
        morris_report.mu_by_output()[1].map(f64::to_bits),
        [2.0_f64.to_bits(), (-3.0_f64).to_bits()]
    );
    assert_eq!(
        morris_report.sigma_by_output()[0].map(f64::to_bits),
        [0.0_f64.to_bits(), 0.0_f64.to_bits()]
    );
    let mut sobol = SobolIndices::<f64, 2, 2>::new();
    for row in 0..16_384_u64 {
        let a0 = Counter::<MultiBaseDomain, SplitMix64>::open_unit::<f64>(seed, row, 0);
        let a1 = Counter::<MultiBaseDomain, SplitMix64>::open_unit::<f64>(seed, row, 1);
        let b0 = Counter::<MultiIndependentDomain, SplitMix64>::open_unit::<f64>(seed, row, 0);
        let b1 = Counter::<MultiIndependentDomain, SplitMix64>::open_unit::<f64>(seed, row, 1);
        sobol.update_outputs(
            &[a0 + a1, 2.0 * a0 - 3.0 * a1],
            &[b0 + b1, 2.0 * b0 - 3.0 * b1],
            &[
                [b0 + a1, a0 + b1],
                [2.0 * b0 - 3.0 * a1, 2.0 * a0 - 3.0 * b1],
            ],
        );
    }
    let sobol_report = sobol.report().expect("defined");
    let first = sobol_report.first_order_by_output();
    let total = sobol_report.total_order_by_output();
    assert!((first[0][0] - 0.5).abs() <= 0.05);
    assert!((first[0][1] - 0.5).abs() <= 0.05);
    assert!((first[1][0] - 4.0 / 13.0).abs() <= 0.05);
    assert!((first[1][1] - 9.0 / 13.0).abs() <= 0.05);
    assert!((total[1][0] - 4.0 / 13.0).abs() <= 0.05);
    assert!((total[1][1] - 9.0 / 13.0).abs() <= 0.05);
}
