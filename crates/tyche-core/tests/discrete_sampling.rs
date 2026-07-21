//! Finite-distribution and importance-sampling laws.

use std::borrow::Cow;

use tyche_core::{
    Categorical, CategoryCount, CategoryIndex, DiscreteImportance, DiscreteWeights,
    ImportanceError, SampleScalar, Seed, SplitMix64, WeightError, WeightedCategorical,
};

const SEED: Seed = Seed::new(0x5459_4348_455F_4341);

#[test]
fn category_values_validate_their_support() {
    let count = CategoryCount::new(3).expect("positive category count");
    assert_eq!(CategoryCount::new(0), None);
    assert_eq!(
        CategoryIndex::new(0, count).map(CategoryIndex::get),
        Some(0)
    );
    assert_eq!(
        CategoryIndex::new(2, count).map(CategoryIndex::get),
        Some(2)
    );
    assert_eq!(CategoryIndex::new(3, count), None);
}

#[test]
fn categorical_replay_matches_the_stream_version() {
    let sampler =
        Categorical::<SplitMix64>::new(CategoryCount::new(7).expect("positive category count"));
    let actual = core::array::from_fn::<_, 12, _>(|index| {
        sampler
            .at(SEED, u64::try_from(index).expect("bounded index"))
            .get()
    });

    assert_eq!(actual, [2, 6, 0, 5, 0, 1, 3, 5, 6, 6, 2, 1]);
    for index in (0_u64..12).rev() {
        assert_eq!(
            sampler.at(SEED, index).get(),
            actual[usize::try_from(index).expect("bounded index")]
        );
    }
}

#[test]
fn categorical_counts_obey_the_uniform_law() {
    const SAMPLES: u64 = 60_000;
    const CATEGORIES: usize = 5;
    let sampler = Categorical::<SplitMix64>::new(
        CategoryCount::new(CATEGORIES).expect("positive category count"),
    );
    let mut counts = [0_u32; CATEGORIES];
    for address in 0..SAMPLES {
        counts[sampler.at(SEED, address).get()] += 1;
    }

    let draw_count = f64::from(u32::try_from(SAMPLES).expect("bounded sample count"));
    let probability = 1.0 / f64::from(u32::try_from(CATEGORIES).expect("bounded count"));
    let expected = draw_count * probability;
    let bound = 6.0 * (draw_count * probability * (1.0 - probability)).sqrt() + 1.0;
    for count in counts {
        let error = (f64::from(count) - expected).abs();
        assert!(error <= bound, "count error {error} exceeds {bound}");
    }
}

fn validate_weight_contract<T: SampleScalar>(largest_finite: T) {
    assert!(matches!(
        DiscreteWeights::<T>::new(Cow::Borrowed(&[] as &[T])),
        Err(WeightError::Empty)
    ));
    assert!(matches!(
        DiscreteWeights::new(Cow::Borrowed(&[T::NAN][..])),
        Err(WeightError::NonFinite { category: 0, .. })
    ));
    assert_eq!(
        DiscreteWeights::new(Cow::Borrowed(&[T::from_f64(-1.0), T::ONE][..])),
        Err(WeightError::Negative {
            category: 0,
            mass: T::from_f64(-1.0),
        })
    );
    assert_eq!(
        DiscreteWeights::new(Cow::Borrowed(&[T::ZERO, T::ZERO][..])),
        Err(WeightError::ZeroTotal)
    );
    assert!(matches!(
        DiscreteWeights::new(Cow::Borrowed(&[largest_finite, largest_finite][..])),
        Err(WeightError::NonFiniteTotal { .. })
    ));

    let target = [T::ONE, T::from_f64(3.0)];
    let proposal = [T::from_f64(2.0), T::from_f64(2.0)];
    let sampler = DiscreteImportance::<T, SplitMix64>::new(
        Cow::Borrowed(&target[..]),
        Cow::Borrowed(&proposal[..]),
    )
    .expect("compatible target and proposal");
    let count = sampler.target().category_count();
    let first = CategoryIndex::new(0, count).expect("category in support");
    let second = CategoryIndex::new(1, count).expect("category in support");

    assert_eq!(sampler.likelihood_ratio(first), Some(T::from_f64(0.5)));
    assert_eq!(sampler.likelihood_ratio(second), Some(T::from_f64(1.5)));

    let target_expectation =
        T::from_f64(0.25) * T::from_f64(2.0) + T::from_f64(0.75) * T::from_f64(6.0);
    let importance_expectation = T::from_f64(0.5)
        * T::from_f64(2.0)
        * sampler.likelihood_ratio(first).expect("positive proposal")
        + T::from_f64(0.5)
            * T::from_f64(6.0)
            * sampler.likelihood_ratio(second).expect("positive proposal");
    assert_eq!(importance_expectation, target_expectation);

    for address in 0..1_024 {
        let sample = sampler.at(SEED, address);
        assert_eq!(
            Some(sample.likelihood_ratio()),
            sampler.likelihood_ratio(sample.category())
        );
    }
}

#[test]
fn weight_and_importance_laws_hold_for_every_supported_primitive_field() {
    validate_weight_contract::<f32>(f32::MAX);
    validate_weight_contract::<f64>(f64::MAX);
}

#[test]
fn borrowed_and_owned_weight_tables_preserve_storage() {
    let borrowed_masses = [1.0_f64, 2.0, 3.0];
    let borrowed =
        DiscreteWeights::new(Cow::Borrowed(&borrowed_masses[..])).expect("valid borrowed masses");
    assert!(core::ptr::eq(
        borrowed.as_slice().as_ptr(),
        borrowed_masses.as_ptr()
    ));

    let owned_masses = vec![3.0_f64, 2.0, 1.0];
    let owned_pointer = owned_masses.as_ptr();
    let owned = DiscreteWeights::new(Cow::Owned(owned_masses)).expect("valid owned masses");
    assert_eq!(owned.as_slice().as_ptr(), owned_pointer);
}

#[test]
fn weighted_sampling_respects_mass_and_zero_support() {
    const SAMPLES: u64 = 60_000;
    let masses = [1.0_f64, 0.0, 3.0];
    let sampler = WeightedCategorical::<f64, SplitMix64>::new(Cow::Borrowed(&masses[..]))
        .expect("valid masses");
    let mut counts = [0_u32; 3];
    for address in 0..SAMPLES {
        counts[sampler.at(SEED, address).get()] += 1;
    }

    assert_eq!(counts[1], 0);
    let draw_count = f64::from(u32::try_from(SAMPLES).expect("bounded sample count"));
    for (count, probability) in [(counts[0], 0.25_f64), (counts[2], 0.75)] {
        let expected = draw_count * probability;
        let bound = 6.0 * (draw_count * probability * (1.0 - probability)).sqrt() + 1.0;
        let error = (f64::from(count) - expected).abs();
        assert!(error <= bound, "count error {error} exceeds {bound}");
    }

    let point_mass =
        WeightedCategorical::<f64, SplitMix64>::new(Cow::Borrowed(&[0.0, 0.0, 4.0, 0.0][..]))
            .expect("valid point mass");
    for address in 0..1_024 {
        assert_eq!(point_mass.at(SEED, address).get(), 2);
    }
}

#[test]
fn importance_validation_rejects_incompatible_supports() {
    assert!(matches!(
        DiscreteImportance::<f64, SplitMix64>::new(
            Cow::Borrowed(&[1.0, 2.0][..]),
            Cow::Borrowed(&[1.0][..]),
        ),
        Err(ImportanceError::CategoryCountMismatch {
            target: 2,
            proposal: 1
        })
    ));
    assert!(matches!(
        DiscreteImportance::<f64, SplitMix64>::new(
            Cow::Borrowed(&[1.0, 2.0][..]),
            Cow::Borrowed(&[1.0, 0.0][..]),
        ),
        Err(ImportanceError::MissingProposalSupport {
            category: 1,
            target_mass: 2.0
        })
    ));
}
