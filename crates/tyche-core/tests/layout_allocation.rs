//! Representation and allocation invariants.

use core::mem::size_of;
use core::num::NonZeroU32;
use std::borrow::Cow;

use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};
use tyche_core::{
    Categorical, CategoryCount, Counter, Design, DigitalShift, DiscreteImportance, DiscreteWeights,
    LatinHypercube, Moments, PopulationVariance, RuntimeSobol, Seed, Sobol, SobolDimensions,
    SobolRange, SplitMix64, StandardNormal, Unscrambled, UserDomain, WeightedCategorical,
};

#[global_allocator]
static ALLOCATOR: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

#[test]
fn static_policies_and_counter_samplers_are_zero_sized() {
    type TestDomain = UserDomain<{ u64::from_le_bytes(*b"alloctst") }>;

    assert_eq!(size_of::<SplitMix64>(), 0);
    assert_eq!(size_of::<TestDomain>(), 0);
    assert_eq!(size_of::<Counter<TestDomain, SplitMix64>>(), 0);
    assert_eq!(size_of::<StandardNormal<f64, SplitMix64>>(), 0);
    assert_eq!(
        size_of::<Categorical<SplitMix64>>(),
        size_of::<CategoryCount>()
    );
    assert_eq!(
        size_of::<WeightedCategorical<'static, f64, SplitMix64>>(),
        size_of::<DiscreteWeights<'static, f64>>()
    );
    assert_eq!(
        size_of::<DiscreteImportance<'static, f64, SplitMix64>>(),
        2 * size_of::<DiscreteWeights<'static, f64>>()
    );
    assert_eq!(size_of::<PopulationVariance>(), 0);
    assert_eq!(size_of::<Unscrambled>(), 0);
    assert_eq!(size_of::<DigitalShift<SplitMix64>>(), size_of::<Seed>());
    assert_eq!(size_of::<Sobol<3, Unscrambled>>(), size_of::<SobolRange>());
    assert_eq!(
        size_of::<RuntimeSobol<Unscrambled>>(),
        size_of::<SobolRange>() + size_of::<SobolDimensions>()
    );
}

#[test]
fn repeated_sampling_and_statistics_allocate_nothing() {
    type TestDomain = UserDomain<{ u64::from_le_bytes(*b"alloctst") }>;

    let design = LatinHypercube::<4, SplitMix64>::new(
        Seed::new(21),
        NonZeroU32::new(128).expect("constant is positive"),
    );
    let mut point = [0.0; 4];
    let sobol_range = SobolRange::new(0, NonZeroU32::new(128).expect("constant is positive"))
        .expect("range is representable");
    let sobol = Sobol::<3, Unscrambled>::new(sobol_range, Unscrambled)
        .expect("three dimensions are supported");
    let runtime_sobol = RuntimeSobol::new(
        SobolDimensions::new(3).expect("three dimensions are supported"),
        sobol_range,
        DigitalShift::<SplitMix64>::new(Seed::new(21)),
    );
    let mut sobol_point = [0.0; 3];
    let mut sobol_rows = [0.0; 128 * 3];
    let mut moments = Moments::new();
    let categorical =
        Categorical::<SplitMix64>::new(CategoryCount::new(4).expect("positive category count"));
    let weighted_masses = [1.0_f64, 2.0, 3.0, 4.0];
    let weighted = WeightedCategorical::<f64, SplitMix64>::new(Cow::Borrowed(&weighted_masses[..]))
        .expect("valid masses");
    let target_masses = [4.0_f64, 3.0, 2.0, 1.0];
    let importance = DiscreteImportance::<f64, SplitMix64>::new(
        Cow::Borrowed(&target_masses[..]),
        Cow::Borrowed(&weighted_masses[..]),
    )
    .expect("compatible target and proposal");

    let region = Region::new(ALLOCATOR);
    for index in 0..design.sample_count() {
        design
            .sample_unit_into(index, &mut point)
            .expect("valid index");
        let index = u64::try_from(index).expect("bounded count");
        point[1] = Counter::<TestDomain, SplitMix64>::unit(Seed::new(21), index, 0);
        point[2] = StandardNormal::<f64, SplitMix64>::at(Seed::new(21), index, 0);
        sobol
            .sample_unit_into(
                usize::try_from(index).expect("bounded index"),
                &mut sobol_point,
            )
            .expect("valid index");
        runtime_sobol
            .sample_unit_slice_into(
                usize::try_from(index).expect("bounded index"),
                &mut sobol_point,
            )
            .expect("valid index and output length");
        let categorical_index = categorical.at(Seed::new(21), index);
        let weighted_index = weighted.at(Seed::new(21), index);
        let importance_sample = importance.at(Seed::new(21), index);
        point[3] = f64::from(
            u32::try_from(categorical_index.get() + weighted_index.get())
                .expect("bounded category sum"),
        ) + importance_sample.likelihood_ratio();
        moments.update(point[0]);
    }
    runtime_sobol
        .sample_unit_rows_into(&mut sobol_rows)
        .expect("row-major output has the exact required length");
    let change = region.change();

    assert_eq!(moments.count(), 128);
    assert_eq!(change.allocations, 0);
    assert_eq!(change.reallocations, 0);
    assert_eq!(change.deallocations, 0);
}
