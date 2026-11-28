//! Ensemble bagging over a retained trainable model.
//!
//! The model fits `y = b x` by least squares, `b = sum(x y) / sum(x^2)`.
//! Coordinates are small integers, so each slope is one correctly rounded
//! quotient of exact integer sums and equals the oracle slope bitwise.

use core::cell::Cell;
use core::error::Error;
use core::fmt;
use core::num::NonZeroU32;

use eunomia::{F32, F64, RealField};
use tyche_core::{Bagging, BaggingError, Bootstrap, Seed, SplitMix64, TrainableModel};

const SEED: Seed = Seed::new(0x5459_4348_455F_4247);
const MEMBERS: u32 = 8;
/// Ordinates `y_i = 2 i + e_i` at abscissae `i`, with small integer `e_i`.
const ORDINATES: [i64; 9] = [1, 1, 6, 4, 9, 11, 11, 18, 15];
const INPUTS: [i64; 2] = [3, 5];

/// Fits `y = b x` and counts its fits.
#[derive(Default)]
struct OriginLine {
    fits: Cell<usize>,
}

#[derive(Debug, PartialEq, Eq)]
enum LineError {
    /// No resampled abscissa is non-zero, so the slope is undefined.
    DegenerateDesign,
}

impl fmt::Display for LineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("no resampled abscissa is non-zero")
    }
}

impl Error for LineError {}

impl<T: RealField> TrainableModel<T> for OriginLine {
    type Observation = (T, T);
    type Input = T;
    type Fitted = T;
    type Error = LineError;

    fn fit(&self, observations: &[(T, T)], resample: &[usize]) -> Result<T, LineError> {
        self.fits.set(self.fits.get() + 1);
        let (mut cross, mut square) = (T::ZERO, T::ZERO);
        for &index in resample {
            let (x, y) = observations[index];
            cross += x * y;
            square += x * x;
        }
        if square == T::ZERO {
            return Err(LineError::DegenerateDesign);
        }
        Ok(cross / square)
    }

    fn predict(&self, fitted: &T, input: &T) -> Result<T, LineError> {
        Ok(*fitted * *input)
    }
}

fn observations<T: RealField>() -> Vec<(T, T)> {
    (0_i64..)
        .zip(ORDINATES)
        .map(|(x, y)| (T::from_integer(x), T::from_integer(y)))
        .collect()
}

/// Slope of member `replicate`, from integer sums over indices drawn one
/// address at a time through `Bootstrap::at`.
fn oracle_slope<T: RealField>(design: Bootstrap<SplitMix64>, replicate: u64) -> T {
    let (mut cross, mut square) = (0_i64, 0_i64);
    for draw in 0..u64::try_from(design.resample_size()).expect("small resample") {
        let index = design.at(SEED, replicate, draw);
        let x = i64::try_from(index).expect("small population");
        cross += x * ORDINATES[index];
        square += x * x;
    }
    T::from_integer(cross) / T::from_integer(square)
}

fn bagged_prediction_is_the_member_mean_and_reuses_fitted_state<T: RealField>() {
    assert_eq!(SplitMix64::VERSION.get(), 1, "replay boundary");
    let design =
        Bootstrap::<SplitMix64>::new(ORDINATES.len(), ORDINATES.len()).expect("positive sizes");
    let bagging = Bagging::new(design, NonZeroU32::new(MEMBERS).expect("positive"));
    let model = OriginLine::default();
    let bagged = bagging
        .fit::<_, T>(&model, SEED, &observations::<T>())
        .expect("every resample has a non-zero abscissa");

    assert_eq!(
        u32::try_from(model.fits.get()),
        Ok(MEMBERS),
        "one fit per member"
    );
    let slopes: Vec<T> = (0..u64::from(MEMBERS))
        .map(|replicate| oracle_slope(design, replicate))
        .collect();
    assert_eq!(bagged.fitted_members(), slopes.as_slice());
    assert!(
        slopes.windows(2).any(|pair| pair[0] != pair[1]),
        "resamples must differ"
    );

    for input in INPUTS.map(T::from_integer) {
        let mut total = T::ZERO;
        let mut largest = T::ZERO;
        for &slope in &slopes {
            let prediction = slope * input;
            total += prediction;
            if prediction > largest {
                largest = prediction;
            }
        }
        let expected = total / T::from_integer(i64::from(MEMBERS));
        let actual = bagged.predict(&input).expect("fitted state is retained");
        assert!(
            (actual - expected).abs() <= aggregation_bound(largest),
            "bagged prediction differs from the member mean"
        );
    }
    assert_eq!(
        u32::try_from(model.fits.get()),
        Ok(MEMBERS),
        "prediction never refits"
    );
}

/// Bound on Welford's mean against the naive sum over `M`, for `M` positive
/// predictions at most `largest`, unit roundoff `u = EPSILON / 2`.
///
/// The naive sum has relative error at most `(M - 1) u` and the division adds
/// `u` (Higham, Accuracy and Stability of Numerical Algorithms, Lemma 3.1).
/// Each Welford step `m += (v - m) / k` rounds the difference, quotient and
/// sum, adding at most `3 u largest` since `|v - m|` and `m` stay below
/// `largest`, and earlier errors shrink by `1 - 1/k`. The total is below
/// `4 M u largest`, so `4 M EPSILON largest` bounds it.
fn aggregation_bound<T: RealField>(largest: T) -> T {
    T::from_integer(4 * i64::from(MEMBERS)) * T::EPSILON * largest
}

#[test]
fn bagged_prediction_is_the_member_mean_and_reuses_fitted_state_for_every_scalar() {
    bagged_prediction_is_the_member_mean_and_reuses_fitted_state::<f32>();
    bagged_prediction_is_the_member_mean_and_reuses_fitted_state::<f64>();
    bagged_prediction_is_the_member_mean_and_reuses_fitted_state::<F32>();
    bagged_prediction_is_the_member_mean_and_reuses_fitted_state::<F64>();
}

#[test]
fn bagging_failures_carry_their_typed_variants() {
    let model = OriginLine::default();
    let design = Bootstrap::<SplitMix64>::new(ORDINATES.len(), 4).expect("positive sizes");
    let empty = Bagging::new(design, NonZeroU32::MIN).fit::<_, f64>(&model, SEED, &[]);
    assert!(matches!(
        empty.err(),
        Some(BaggingError::PopulationLength {
            expected: 9,
            actual: 0
        })
    ));
    assert_eq!(model.fits.get(), 0, "no member fits a mismatched set");

    let design = Bootstrap::<SplitMix64>::new(3, 3).expect("positive sizes");
    let bagging = Bagging::new(design, NonZeroU32::new(MEMBERS).expect("positive"));
    let at_origin = bagging.fit::<_, f64>(&model, SEED, &[(0.0, 1.0); 3]);
    assert!(matches!(
        at_origin.err(),
        Some(BaggingError::Model(LineError::DegenerateDesign))
    ));
    assert_eq!(model.fits.get(), 1, "the first failing member ends the fit");
}

/// Fits one observation under a resample of `resample_size` indices and
/// returns the failure with the number of fits the model performed.
fn fit_one_observation<T: RealField>(
    resample_size: usize,
) -> (Option<BaggingError<LineError>>, usize) {
    let model = OriginLine::default();
    let design = Bootstrap::<SplitMix64>::new(1, resample_size).expect("positive sizes");
    let outcome = Bagging::new(design, NonZeroU32::MIN)
        .fit::<_, T>(&model, SEED, &[(T::ONE, T::ONE)])
        .err();
    (outcome, model.fits.get())
}

fn unallocatable_resamples_report_their_size<T: RealField>() {
    assert_eq!(
        fit_one_observation::<T>(1),
        (None, 1),
        "a resample that fits is fitted"
    );
    // `usize::MAX` overflows the capacity computation. The second size is the
    // largest element count whose byte length is at most `isize::MAX`: the
    // capacity check admits it and the allocator refuses it because no host
    // address space holds 2^63 bytes.
    let largest_layout = isize::MAX.cast_unsigned() / size_of::<usize>();
    for resample_size in [usize::MAX, largest_layout] {
        assert_eq!(
            fit_one_observation::<T>(resample_size),
            (Some(BaggingError::ResampleAllocation { resample_size }), 0),
            "no member fits when the resample cannot be allocated"
        );
    }
}

#[test]
fn fit_rejects_an_unallocatable_resample_with_a_typed_error() {
    unallocatable_resamples_report_their_size::<f32>();
    unallocatable_resamples_report_their_size::<f64>();
    unallocatable_resamples_report_their_size::<F32>();
    unallocatable_resamples_report_their_size::<F64>();
}

/// Trained state of `2^17` bytes. `u32::MAX` of them need `2^49` bytes, 512
/// TiB: below `isize::MAX`, so the capacity check admits the request, and
/// above the user address space of 64-bit x86-64 (128 TiB) and aarch64 (256
/// TiB), so the allocator refuses it. A state is returned by value, so the
/// size is bounded by the test thread's stack, which rules out a size whose
/// product overflows `Layout`.
struct Oversized([u8; 1 << 17]);

/// Counts fits; never produces a state, as none could be stored.
#[derive(Default)]
struct RefusedFit {
    fits: Cell<usize>,
}

impl<T: RealField> TrainableModel<T> for RefusedFit {
    type Observation = ();
    type Input = ();
    type Fitted = Oversized;
    type Error = LineError;

    fn fit(&self, _: &[()], _: &[usize]) -> Result<Oversized, LineError> {
        self.fits.set(self.fits.get() + 1);
        Err(LineError::DegenerateDesign)
    }

    fn predict(&self, fitted: &Oversized, (): &()) -> Result<T, LineError> {
        Ok(T::from_integer(i64::from(fitted.0[0])))
    }
}

fn unallocatable_member_storage_reports_the_member_count<T: RealField>() {
    let model = RefusedFit::default();
    let design = Bootstrap::<SplitMix64>::new(1, 1).expect("positive sizes");
    let members = NonZeroU32::MAX;
    let outcome = Bagging::new(design, members)
        .fit::<_, T>(&model, SEED, &[()])
        .err();
    assert_eq!(
        outcome,
        Some(BaggingError::MemberAllocation {
            members: members.get()
        })
    );
    assert_eq!(model.fits.get(), 0, "no member fits without storage");
}

#[test]
fn fit_rejects_unallocatable_member_storage_with_a_typed_error() {
    unallocatable_member_storage_reports_the_member_count::<f32>();
    unallocatable_member_storage_reports_the_member_count::<f64>();
    unallocatable_member_storage_reports_the_member_count::<F32>();
    unallocatable_member_storage_reports_the_member_count::<F64>();
}

fn over_length_observations_report_the_population_length<T: RealField>() {
    let model = OriginLine::default();
    let design =
        Bootstrap::<SplitMix64>::new(ORDINATES.len(), ORDINATES.len()).expect("positive sizes");
    let mut data = observations::<T>();
    data.push((T::ONE, T::ONE));
    let outcome = Bagging::new(design, NonZeroU32::MIN)
        .fit::<_, T>(&model, SEED, &data)
        .err();
    assert_eq!(
        outcome,
        Some(BaggingError::PopulationLength {
            expected: ORDINATES.len(),
            actual: ORDINATES.len() + 1
        })
    );
    assert_eq!(model.fits.get(), 0, "no member fits a mismatched set");
}

#[test]
fn over_length_observations_report_population_length() {
    over_length_observations_report_the_population_length::<f32>();
    over_length_observations_report_the_population_length::<f64>();
    over_length_observations_report_the_population_length::<F32>();
    over_length_observations_report_the_population_length::<F64>();
}

#[test]
fn bagging_failures_display_their_context() {
    let failures = [
        (
            BaggingError::<LineError>::PopulationLength {
                expected: 9,
                actual: 10,
            },
            "bagging received 10 observations for a population of 9",
        ),
        (
            BaggingError::ResampleAllocation { resample_size: 5 },
            "bagging cannot allocate a resample of 5 indices",
        ),
        (
            BaggingError::MemberAllocation { members: 6 },
            "bagging cannot allocate storage for 6 trained states",
        ),
        (
            BaggingError::Model(LineError::DegenerateDesign),
            "bagging member fit failed",
        ),
    ];
    for (failure, text) in failures {
        assert_eq!(failure.to_string(), text);
    }
}

#[test]
fn model_failure_exposes_its_source() {
    let failure = BaggingError::Model(LineError::DegenerateDesign);
    let source = failure.source().expect("a member failure is the source");
    assert_eq!(
        source.downcast_ref::<LineError>(),
        Some(&LineError::DegenerateDesign)
    );
    let own_failures = [
        BaggingError::<LineError>::PopulationLength {
            expected: 9,
            actual: 10,
        },
        BaggingError::ResampleAllocation { resample_size: 1 },
        BaggingError::MemberAllocation { members: 1 },
    ];
    for failure in own_failures {
        assert_eq!(
            failure.source().map(ToString::to_string),
            None,
            "{failure} has no source"
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rejected {
    member: usize,
}

/// Numbers its members by fit order; every member from `first_rejected` on
/// rejects its prediction, naming itself.
struct RejectingPredict {
    first_rejected: usize,
    fits: Cell<usize>,
}

impl<T: RealField> TrainableModel<T> for RejectingPredict {
    type Observation = ();
    type Input = ();
    type Fitted = usize;
    type Error = Rejected;

    fn fit(&self, _: &[()], _: &[usize]) -> Result<usize, Rejected> {
        let member = self.fits.get();
        self.fits.set(member + 1);
        Ok(member)
    }

    fn predict(&self, fitted: &usize, (): &()) -> Result<T, Rejected> {
        if *fitted < self.first_rejected {
            Ok(T::ONE)
        } else {
            Err(Rejected { member: *fitted })
        }
    }
}

fn prediction_failure_is_the_first_rejecting_member<T: RealField>() {
    let design = Bootstrap::<SplitMix64>::new(1, 1).expect("positive sizes");
    let members = usize::try_from(MEMBERS).expect("small member count");
    for first_rejected in 0..=members {
        let model = RejectingPredict {
            first_rejected,
            fits: Cell::new(0),
        };
        let bagged = Bagging::new(design, NonZeroU32::new(MEMBERS).expect("positive"))
            .fit::<_, T>(&model, SEED, &[()])
            .expect("fitting never fails");
        let outcome = bagged.predict(&());
        if first_rejected == members {
            // `Moments` starts its mean at zero (`moments.rs:38`), so the first
            // update adds `(1 - 0) / 1 = 1` exactly and every later update
            // adds `(1 - 1) / k = 0` (`moments.rs:56-57`): the mean is exactly one.
            assert_eq!(outcome, Ok(T::ONE), "no member rejects");
        } else {
            assert_eq!(
                outcome,
                Err(Rejected {
                    member: first_rejected
                }),
                "the first rejecting member's failure is returned"
            );
        }
    }
}

#[test]
fn prediction_failure_propagates_from_the_first_member() {
    prediction_failure_is_the_first_rejecting_member::<f32>();
    prediction_failure_is_the_first_rejecting_member::<f64>();
    prediction_failure_is_the_first_rejecting_member::<F32>();
    prediction_failure_is_the_first_rejecting_member::<F64>();
}
