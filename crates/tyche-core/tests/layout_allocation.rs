//! Representation and allocation invariants.

use core::mem::size_of;
use core::num::NonZeroU32;

use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};
use tyche_core::{
    Design, LatinHypercube, Moments, PopulationVariance, Seed, SplitMix64, StandardNormal,
};

#[global_allocator]
static ALLOCATOR: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

#[test]
fn static_policies_and_counter_samplers_are_zero_sized() {
    assert_eq!(size_of::<SplitMix64>(), 0);
    assert_eq!(size_of::<StandardNormal>(), 0);
    assert_eq!(size_of::<PopulationVariance>(), 0);
}

#[test]
fn repeated_sampling_and_statistics_allocate_nothing() {
    let design = LatinHypercube::<4>::new(
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
        moments.update(point[0]);
    }
    let change = region.change();

    assert_eq!(moments.count(), 128);
    assert_eq!(change.allocations, 0);
    assert_eq!(change.reallocations, 0);
    assert_eq!(change.deallocations, 0);
}
