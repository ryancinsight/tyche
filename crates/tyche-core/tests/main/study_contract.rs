//! Study, GAT, Cow, layout, and allocation contracts.

use core::mem::size_of;
use core::num::NonZeroU32;
use tyche_core::{
    Design, LatinHypercube, Moments, Parameter, ParameterSpace, PopulationVariance,
    ResponseReducer, Seed, SplitMix64, StandardNormal, Study, StudyModel,
};

#[path = "../fixtures/mod.rs"]
mod fixtures;

use fixtures::{BorrowingModel, CopyResponse};

#[test]
fn borrowing_and_allocation_contracts_hold() {
    let source = "flow";
    let space = ParameterSpace::new([
        Parameter::borrowed(source, 1.0_f64, 3.0).expect("valid"),
        Parameter::borrowed("pressure", 10.0, 20.0).expect("valid"),
    ])
    .expect("unique");
    let design =
        LatinHypercube::<2, SplitMix64>::new(Seed::new(9), NonZeroU32::new(128).expect("positive"));
    let study = Study::borrowed("pump", space, design).expect("named");
    assert!(core::ptr::eq(
        study.space().parameters()[0].name().as_ptr(),
        source.as_ptr()
    ));
    assert_eq!(size_of::<SplitMix64>(), 0);
    assert_eq!(size_of::<StandardNormal<f64, SplitMix64>>(), 0);
    assert_eq!(size_of::<PopulationVariance>(), 0);

    let mut point = [0.0; 2];
    let mut moments = Moments::new();
    let allocations = allocation_counter::measure(|| {
        for index in 0..study.sample_count() {
            study
                .design()
                .sample_unit_into(index, &mut point)
                .expect("valid");
            moments.update(point[0]);
        }
    });
    assert_eq!(allocations, allocation_counter::AllocationInfo::default());

    let sample = study.sample(3).expect("valid");
    let response = BorrowingModel::<0>
        .evaluate(sample.values())
        .expect("infallible");
    assert!(core::ptr::eq(
        core::ptr::from_ref(response),
        core::ptr::from_ref(&sample.values()[0])
    ));
    assert_eq!(
        <CopyResponse as ResponseReducer<BorrowingModel<0>, f64, 2>>::reduce(
            &CopyResponse,
            response
        )
        .to_bits(),
        sample.values()[0].to_bits()
    );
}
