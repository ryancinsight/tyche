//! Executable evidence for finite-sample conformal calibration.

use tyche_core::ConformalCalibrator;

#[test]
fn corrected_finite_sample_rank_selects_upper_tail() {
    let calibrator = ConformalCalibrator::new(0.1_f64).expect("valid alpha");
    let mut scores = [0.5, 0.1, 0.3, 0.2, 0.4];

    let radius = calibrator
        .calibrate_in_place(&mut scores)
        .expect("valid scores");

    assert_eq!(radius, 0.5);
    assert_eq!(scores, [0.1, 0.2, 0.3, 0.4, 0.5]);
}

#[test]
fn median_miscoverage_uses_correct_one_based_rank() {
    let calibrator = ConformalCalibrator::new(0.5_f32).expect("valid alpha");
    let mut scores = [0.1_f32, 0.5, 0.2, 0.4, 0.3];
    assert_eq!(
        calibrator
            .calibrate_in_place(&mut scores)
            .expect("valid scores"),
        0.3
    );
}

#[test]
fn invalid_scores_are_rejected_before_sorting() {
    let calibrator = ConformalCalibrator::new(0.05_f64).expect("valid alpha");
    let mut scores = [0.2, f64::NAN, 0.1];
    let before = scores.map(f64::to_bits);

    let error = calibrator
        .calibrate_in_place(&mut scores)
        .expect_err("NaN is not a nonconformity score");

    assert!(matches!(
        error,
        tyche_core::ConformalError::InvalidScore { index: 1, .. }
    ));
    assert_eq!(scores.map(f64::to_bits), before);
}
