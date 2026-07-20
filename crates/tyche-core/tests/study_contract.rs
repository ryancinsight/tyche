//! Study, GAT, Cow, layout, and allocation contracts.

use core::mem::size_of;
use core::num::NonZeroU32;
use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};
use tyche_core::{
    Design, LatinHypercube, Moments, Parameter, ParameterSpace, PopulationVariance,
    ResponseReducer, Seed, SplitMix64, StandardNormal, Study, StudyModel,
};

#[global_allocator]
static ALLOCATOR: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

struct BorrowingModel;
impl StudyModel<f64, 2> for BorrowingModel {
    type Error = core::convert::Infallible;
    type Response<'a> = &'a f64;
    fn evaluate<'a>(&'a self, parameters: &'a [f64; 2]) -> Result<Self::Response<'a>, Self::Error> {
        Ok(&parameters[0])
    }
}
struct CopyResponse;
impl ResponseReducer<BorrowingModel, f64, 2> for CopyResponse {
    type Output = f64;
    fn reduce<'a>(
        &self,
        response: <BorrowingModel as StudyModel<f64, 2>>::Response<'a>,
    ) -> Self::Output
    where
        BorrowingModel: 'a,
        f64: 'a,
    {
        *response
    }
}

#[test]
fn borrowing_and_allocation_contracts_hold() {
    let source = "flow";
    let space = ParameterSpace::new([
        Parameter::borrowed(source, 1.0_f64, 3.0).expect("valid"),
        Parameter::borrowed("pressure", 10.0, 20.0).expect("valid"),
    ])
    .expect("unique");
    let design = LatinHypercube::new(Seed::new(9), NonZeroU32::new(128).expect("positive"));
    let study = Study::borrowed("pump", space, design).expect("named");
    assert!(core::ptr::eq(
        study.space().parameters()[0].name().as_ptr(),
        source.as_ptr()
    ));
    assert_eq!(size_of::<SplitMix64>(), 0);
    assert_eq!(size_of::<StandardNormal>(), 0);
    assert_eq!(size_of::<PopulationVariance>(), 0);

    let mut point = [0.0; 2];
    let mut moments = Moments::new();
    let region = Region::new(ALLOCATOR);
    for index in 0..study.sample_count() {
        study
            .design()
            .sample_unit_into(index, &mut point)
            .expect("valid");
        moments.update(point[0]);
    }
    let change = region.change();
    assert_eq!(change.allocations, 0);
    assert_eq!(change.reallocations, 0);
    assert_eq!(change.deallocations, 0);

    let sample = study.sample(3).expect("valid");
    let response = BorrowingModel
        .evaluate(sample.values())
        .expect("infallible");
    assert!(core::ptr::eq(response, &sample.values()[0]));
    assert_eq!(CopyResponse.reduce(response), sample.values()[0]);
}
