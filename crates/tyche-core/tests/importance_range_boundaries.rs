//! Public importance-ratio boundary, validation and ordinary-bit contracts.

use std::borrow::Cow;
use tyche_core::{
    CategoryCount, CategoryIndex, DiscreteImportance, ImportanceError, SampleScalar, Seed,
    SplitMix64, WeightError,
};

fn ratio<T: SampleScalar>(target: &[T], proposal: &[T]) -> T {
    let sampler =
        DiscreteImportance::<T, SplitMix64>::new(Cow::Borrowed(target), Cow::Borrowed(proposal))
            .expect("finite supported ratios");
    let category = CategoryIndex::new(0, sampler.target().category_count()).expect("in support");
    sampler
        .likelihood_ratio(category)
        .expect("positive proposal")
}

fn boundaries<T: SampleScalar>(small: T, largest: T) {
    let one = T::ONE;
    let two = one + one;
    let four = two + two;
    let eight = four + four;
    let three_small = small + small + small;
    assert_eq!(ratio(&[small, four], &[one, one]), T::ZERO); // half ties even
    assert_eq!(ratio(&[three_small, four], &[one, one]), small + small);
    assert_eq!(ratio(&[small, eight], &[one, one]), T::ZERO); // below half
    assert_eq!(ratio(&[three_small, eight], &[one, one]), small); // above half

    let precision = 1 - T::EPSILON.binary_exponent().expect("positive epsilon");
    let normal = small.scale_binary(precision - 1);
    for value in [normal - small, normal, normal + small] {
        assert_eq!(ratio(&[value, two], &[one, one]), value);
    }
    assert_eq!(ratio(&[one, T::ZERO], &[one, largest]), largest);
    let previous_one = T::unit_from_word(u64::MAX);
    let error = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&[one, T::ZERO][..]),
        Cow::Borrowed(&[previous_one, largest][..]),
    )
    .expect_err("the exact ratio is beyond the finite range");
    assert!(matches!(
        error,
        ImportanceError::NonFiniteLikelihoodRatio { category: 0, .. }
    ));
}

fn validation<T: SampleScalar>(small: T, largest: T) {
    let one = T::ONE;
    let zero = T::ZERO;
    let good = [one, one];
    for mass in [T::NAN, T::INFINITY, -T::INFINITY] {
        let invalid = [mass, one];
        let error = DiscreteImportance::<T, SplitMix64>::new(
            Cow::Borrowed(&invalid[..]),
            Cow::Borrowed(&good[..]),
        )
        .expect_err("nonfinite target mass");
        assert!(matches!(
            error,
            ImportanceError::TargetWeights(WeightError::NonFinite { category: 0, .. })
        ));
        let error = DiscreteImportance::<T, SplitMix64>::new(
            Cow::Borrowed(&good[..]),
            Cow::Borrowed(&invalid[..]),
        )
        .expect_err("nonfinite proposal mass");
        assert!(matches!(
            error,
            ImportanceError::ProposalWeights(WeightError::NonFinite { category: 0, .. })
        ));
    }
    let overflow = [largest, largest];
    let error = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&overflow[..]),
        Cow::Borrowed(&good[..]),
    )
    .expect_err("native total overflow");
    assert!(matches!(
        error,
        ImportanceError::TargetWeights(WeightError::NonFiniteTotal { .. })
    ));
    let error = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&[zero, zero][..]),
        Cow::Borrowed(&good[..]),
    )
    .expect_err("zero total");
    assert_eq!(
        error,
        ImportanceError::TargetWeights(WeightError::ZeroTotal)
    );
    let error = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&[-one, one][..]),
        Cow::Borrowed(&good[..]),
    )
    .expect_err("negative mass");
    assert_eq!(
        error,
        ImportanceError::TargetWeights(WeightError::Negative {
            category: 0,
            mass: -one
        })
    );
    let error = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&good[..]),
        Cow::Borrowed(&[one][..]),
    )
    .expect_err("count mismatch");
    assert_eq!(
        error,
        ImportanceError::CategoryCountMismatch {
            target: 2,
            proposal: 1
        }
    );
    let error = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&good[..]),
        Cow::Borrowed(&[zero, one][..]),
    )
    .expect_err("missing positive target support");
    assert_eq!(
        error,
        ImportanceError::MissingProposalSupport {
            category: 0,
            target_mass: one
        }
    );
    assert_eq!(ratio(&[zero, one], &[small, one + one]), zero);
    let negative_zero = ratio(&[-zero, one], &[small, one + one]);
    assert_eq!(negative_zero, zero);
    assert!(negative_zero.is_sign_negative());
    let matching = [zero, one];
    let sampler = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&matching[..]),
        Cow::Borrowed(&matching[..]),
    )
    .expect("matching zero supports");
    let zero_category =
        CategoryIndex::new(0, sampler.target().category_count()).expect("in support");
    assert_eq!(sampler.likelihood_ratio(zero_category), None);
    let outside =
        CategoryIndex::new(2, CategoryCount::new(3).expect("nonempty")).expect("other support");
    assert_eq!(sampler.likelihood_ratio(outside), None);
    for address in 0..128 {
        let sample = sampler.at(Seed::new(17), address);
        assert_eq!(sample.category().get(), 1);
        assert_eq!(sample.likelihood_ratio(), one);
    }
}

fn ordinary_bits<T: SampleScalar>(bits: fn(T) -> u64) {
    for a in 1..=8 {
        for b in 1..=8 {
            for c in 1..=8 {
                for d in 1..=8 {
                    let target = [T::from_integer(a), T::from_integer(b)];
                    let proposal = [T::from_integer(c), T::from_integer(d)];
                    let expected = (target[0] / (target[0] + target[1]))
                        / (proposal[0] / (proposal[0] + proposal[1]));
                    assert_eq!(bits(ratio(&target, &proposal)), bits(expected));
                }
            }
        }
    }
    assert_eq!(
        bits(ratio(&[-T::ZERO, T::ONE], &[T::ONE, T::ONE])),
        bits(-T::ZERO)
    );
}

#[test]
fn boundary_rounding_f32() {
    boundaries(f32::from_bits(1), f32::MAX);
}
#[test]
fn boundary_rounding_f64() {
    boundaries(f64::from_bits(1), f64::MAX);
}
#[test]
fn invalid_inputs_and_support_f32() {
    validation(f32::from_bits(1), f32::MAX);
}
#[test]
fn invalid_inputs_and_support_f64() {
    validation(f64::from_bits(1), f64::MAX);
}
#[test]
fn ordinary_path_bits_f32() {
    ordinary_bits(|v: f32| u64::from(v.to_bits()));
}
#[test]
fn ordinary_path_bits_f64() {
    ordinary_bits(f64::to_bits);
}
