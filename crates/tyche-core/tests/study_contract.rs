//! Study composition, GAT, and ownership contracts.

use core::num::NonZeroUsize;

use tyche_core::{
    LatinHypercube, Parameter, ParameterSpace, ResponseReducer, Seed, Study, StudyModel,
};

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

    fn reduce<'a>(&self, response: &'a f64) -> Self::Output {
        *response
    }
}

#[test]
fn borrowed_names_and_borrowed_gat_responses_preserve_identity() {
    let first_name = "flow";
    let space = ParameterSpace::new([
        Parameter::borrowed(first_name, 1.0_f64, 3.0).expect("valid interval"),
        Parameter::borrowed("pressure", 10.0, 20.0).expect("valid interval"),
    ])
    .expect("unique parameter names");
    let design = LatinHypercube::new(
        Seed::new(9),
        NonZeroUsize::new(8).expect("constant is positive"),
    );
    let study = Study::borrowed("pump sweep", space, design).expect("named study");
    let sample = study.sample(3).expect("valid sample");
    let response = BorrowingModel
        .evaluate(sample.values())
        .expect("infallible model");

    assert!(study.is_name_borrowed());
    assert!(study.space().parameters()[0].is_name_borrowed());
    assert!(core::ptr::eq(
        study.space().parameters()[0].name().as_ptr(),
        first_name.as_ptr()
    ));
    assert!(core::ptr::eq(response, &sample.values()[0]));
    assert_eq!(CopyResponse.reduce(response), sample.values()[0]);
}

#[test]
fn mapped_samples_respect_parameter_intervals() {
    let space = ParameterSpace::new([
        Parameter::borrowed("x", -2.0_f32, -1.0).expect("valid interval"),
        Parameter::borrowed("y", 5.0, 9.0).expect("valid interval"),
    ])
    .expect("unique names");
    let design = LatinHypercube::new(
        Seed::new(12),
        NonZeroUsize::new(64).expect("constant is positive"),
    );
    let study = Study::borrowed("bounds", space, design).expect("named study");

    for index in 0..study.sample_count() {
        let values = study.sample(index).expect("valid index").into_values();
        assert!((-2.0..-1.0).contains(&values[0]));
        assert!((5.0..9.0).contains(&values[1]));
    }
}
