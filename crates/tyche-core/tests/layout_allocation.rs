//! Representation and allocation invariants.

use core::mem::size_of;
use core::num::NonZeroU32;

use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};
use tyche_core::{
    Counter, Design, LatinHypercube, Moments, PopulationVariance, Seed, SplitMix64, StandardNormal,
    UserDomain,
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
    assert_eq!(size_of::<PopulationVariance>(), 0);
}

#[test]
fn repeated_sampling_and_statistics_allocate_nothing() {
    type TestDomain = UserDomain<{ u64::from_le_bytes(*b"alloctst") }>;

    let design = LatinHypercube::<4, SplitMix64>::new(
        Seed::new(21),
        NonZeroU32::new(128).expect("constant is positive"),
    );
    let mut point = [0.0; 4];
    let mut moments = Moments::new();

    let region = Region::new(ALLOCATOR);
    for index in 0..design.sample_count() {
        design
            .sample_unit_into(index, &mut point)
            .expect("valid index");
        let index = u64::try_from(index).expect("bounded count");
        point[1] = Counter::<TestDomain, SplitMix64>::unit(Seed::new(21), index, 0);
        point[2] = StandardNormal::<f64, SplitMix64>::at(Seed::new(21), index, 0);
        moments.update(point[0]);
    }
    let change = region.change();

    assert_eq!(moments.count(), 128);
    assert_eq!(change.allocations, 0);
    assert_eq!(change.reallocations, 0);
    assert_eq!(change.deallocations, 0);
}
