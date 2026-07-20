//! Conformal-rank evidence.

use tyche_core::ConformalCalibrator;

fn corrected_rank_selects_upper_tail<T: eunomia::RealField>() {
    let calibrator = ConformalCalibrator::new(T::from_f64(0.1)).expect("valid");
    let mut scores = [
        T::from_f64(0.5),
        T::from_f64(0.1),
        T::from_f64(0.3),
        T::from_f64(0.2),
        T::from_f64(0.4),
    ];
    assert_eq!(
        calibrator.calibrate_in_place(&mut scores).expect("valid"),
        T::from_f64(0.5)
    );
    assert_eq!(
        scores,
        [
            T::from_f64(0.1),
            T::from_f64(0.2),
            T::from_f64(0.3),
            T::from_f64(0.4),
            T::from_f64(0.5),
        ]
    );
}

#[test]
fn corrected_rank_is_native_for_every_supported_primitive_field() {
    corrected_rank_selects_upper_tail::<f32>();
    corrected_rank_selects_upper_tail::<f64>();
}

#[test]
fn sorted_calibration_borrows_without_mutation() {
    let calibrator = ConformalCalibrator::new(0.1_f32).expect("valid");
    let scores = [0.1_f32, 0.2, 0.3, 0.4, 0.5];
    let before = scores.map(f32::to_bits);
    assert_eq!(
        calibrator
            .calibrate_sorted(&scores)
            .expect("valid")
            .to_bits(),
        0.5_f32.to_bits()
    );
    assert_eq!(scores.map(f32::to_bits), before);
}

#[test]
fn sorted_calibration_rejects_decreasing_scores() {
    let calibrator = ConformalCalibrator::new(0.1_f64).expect("valid");
    let scores = [0.1_f64, 0.3, 0.2];
    assert_eq!(
        calibrator.calibrate_sorted(&scores),
        Err(tyche_core::ConformalError::InvalidScore {
            index: 2,
            value: 0.2,
        })
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
