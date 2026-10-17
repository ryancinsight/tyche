//! Reproduction-only native likelihood-ratio regression and positive controls.

use std::borrow::Cow;
use tyche_core::{
    CategoryIndex, DiscreteImportance, ImportanceError, SampleScalar, Seed, SplitMix64,
    WeightedCategorical,
};

fn controls<T: SampleScalar>(tiny: T) {
    assert!(tiny > T::ZERO);
    assert_eq!(T::unit_from_word(0), T::ZERO);
    assert!(T::unit_from_word(u64::MAX) < T::ONE);
    assert!(T::open_unit_from_word(0) > T::ZERO);

    // Ordinary and adjacent representable probability boundaries retain P/P=1.
    for masses in [
        [T::ONE, T::from_f64(2.0)],
        [tiny, T::ONE],
        [tiny + tiny, T::from_f64(2.0)],
    ] {
        let sampler = DiscreteImportance::<T, SplitMix64>::new(
            Cow::Borrowed(&masses[..]),
            Cow::Borrowed(&masses[..]),
        )
        .expect("ordinary/adjacent identical distributions are valid");
        let count = sampler.target().category_count();
        for index in 0..2 {
            let category = CategoryIndex::new(index, count).expect("in support");
            assert_eq!(sampler.likelihood_ratio(category), Some(T::ONE));
        }
    }

    // The zero-tail/subnormal inverse-CDF fallback preserves positive support.
    let point_mass = [T::ZERO, tiny, T::ZERO];
    let sampler = WeightedCategorical::<T, SplitMix64>::new(Cow::Borrowed(&point_mass[..]))
        .expect("one positive subnormal mass is a valid distribution");
    assert_eq!(sampler.weights().total(), tiny);
    for address in 0..1024 {
        assert_eq!(sampler.at(Seed::new(3), address).get(), 1);
    }

    // This likelihood ratio really is outside T's finite range; retain rejection.
    let target = [T::ONE, T::ONE];
    let proposal = [tiny, T::from_f64(2.0)];
    let error = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&target[..]),
        Cow::Borrowed(&proposal[..]),
    )
    .expect_err("the first exact ratio exceeds the primitive's finite range");
    assert!(matches!(
        error,
        ImportanceError::NonFiniteLikelihoodRatio { category: 0, .. }
    ));
}

fn identical_tiny<T: SampleScalar>(tiny: T) {
    let masses = [tiny, T::from_f64(2.0)];
    let sampler = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&masses[..]),
        Cow::Borrowed(&masses[..]),
    )
    .expect("identical positive distributions have representable likelihood ratio one");
    let first = CategoryIndex::new(0, sampler.target().category_count()).expect("in support");
    assert_eq!(sampler.likelihood_ratio(first), Some(T::ONE));
}

fn representable_tiny<T: SampleScalar>(tiny: T) {
    let target = [tiny, T::from_f64(2.0)];
    let proposal = [T::ONE, T::from_f64(3.0)];
    let sampler = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&target[..]),
        Cow::Borrowed(&proposal[..]),
    )
    .expect("valid target/proposal and finite representable ratios");
    let first = CategoryIndex::new(0, sampler.target().category_count()).expect("in support");
    let actual = sampler
        .likelihood_ratio(first)
        .expect("positive proposal mass");
    let draw = (0..128)
        .map(|address| sampler.at(Seed::new(3), address))
        .find(|sample| sample.category() == first)
        .expect("fixed replay visits first category");
    assert_eq!(draw.likelihood_ratio(), actual);
    // Exact ratio = 4*tiny/(2+tiny), whose correctly rounded value is 2*tiny.
    let expected = tiny + tiny;
    assert!(expected > T::ZERO);
    assert_eq!(
        actual, expected,
        "normalization underflow must not erase a representable ratio"
    );
}

#[test]
fn ordinary_support_and_adjacent_controls_f32() {
    controls(f32::from_bits(1));
}
#[test]
fn ordinary_support_and_adjacent_controls_f64() {
    controls(f64::from_bits(1));
}
#[test]
fn identical_subnormal_distribution_is_valid_f32() {
    identical_tiny(f32::from_bits(1));
}
#[test]
fn identical_subnormal_distribution_is_valid_f64() {
    identical_tiny(f64::from_bits(1));
}
#[test]
fn finite_subnormal_likelihood_is_not_zero_f32() {
    representable_tiny(f32::from_bits(1));
}
#[test]
fn finite_subnormal_likelihood_is_not_zero_f64() {
    representable_tiny(f64::from_bits(1));
}
