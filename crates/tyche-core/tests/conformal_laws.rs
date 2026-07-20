//! Conformal-rank evidence.

use tyche_core::ConformalCalibrator;

#[test]
fn corrected_rank_selects_upper_tail() {
    let calibrator = ConformalCalibrator::new(0.1_f64).expect("valid");
    let mut scores = [0.5, 0.1, 0.3, 0.2, 0.4];
    assert_eq!(
        calibrator
            .calibrate_in_place(&mut scores)
            .expect("valid")
            .to_bits(),
        0.5_f64.to_bits()
    );
    assert_eq!(
        scores.map(f64::to_bits),
        [0.1_f64, 0.2, 0.3, 0.4, 0.5].map(f64::to_bits)
    );
}

#[test]
fn invalid_scores_are_rejected_before_sorting() {
    let calibrator = ConformalCalibrator::new(0.05_f64).expect("valid");
    let mut scores = [0.2, f64::NAN, 0.1];
    let before = scores.map(f64::to_bits);
    assert!(calibrator.calibrate_in_place(&mut scores).is_err());
    assert_eq!(scores.map(f64::to_bits), before);
}
