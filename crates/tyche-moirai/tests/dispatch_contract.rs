//! Moirai adapter contracts.

use core::num::NonZeroU32;
use moirai_core::executor::ExecutorConfig;
use moirai_executor::HybridExecutor;
use tyche_core::{
    Design, LatinHypercube, Parameter, ParameterSpace, ResponseReducer, SampleIndexError, Seed,
    SplitMix64, Study, StudyModel,
};
use tyche_moirai::{DispatchError, MoiraiDispatch};

struct BorrowingModel;
impl StudyModel<f64, 2> for BorrowingModel {
    type Error = core::convert::Infallible;
    type Response<'a> = &'a f64;
    fn evaluate<'a>(&'a self, parameters: &'a [f64; 2]) -> Result<Self::Response<'a>, Self::Error> {
        Ok(&parameters[1])
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

struct RejectingDesign;

impl Design<2> for RejectingDesign {
    fn sample_count(&self) -> usize {
        3
    }

    fn sample_unit_into(
        &self,
        index: usize,
        output: &mut [f64; 2],
    ) -> Result<(), SampleIndexError> {
        if index == 1 || index >= self.sample_count() {
            return Err(SampleIndexError::new(index, self.sample_count()));
        }
        *output = [0.25, 0.75];
        Ok(())
    }
}

#[test]
fn dispatch_preserves_logical_indices() {
    let space = ParameterSpace::new([
        Parameter::borrowed("x", 0.0, 1.0).expect("valid"),
        Parameter::borrowed("y", 10.0, 20.0).expect("valid"),
    ])
    .expect("unique");
    let design = LatinHypercube::<2, SplitMix64>::new(
        Seed::new(44),
        NonZeroU32::new(257).expect("positive"),
    );
    let study = Study::borrowed("parallel", space, design).expect("named");
    let mut executor = HybridExecutor::new(ExecutorConfig {
        worker_threads: 2,
        ..ExecutorConfig::default()
    })
    .expect("executor");
    let mut output: Vec<Option<Result<f64, core::convert::Infallible>>> =
        (0..257).map(|_| None).collect();
    MoiraiDispatch::<7>::new(&executor)
        .evaluate_into(&study, &BorrowingModel, &CopyResponse, &mut output)
        .expect("dispatch");
    for (index, slot) in output.into_iter().enumerate() {
        let actual = slot.expect("initialized").expect("infallible");
        assert_eq!(
            actual.to_bits(),
            study.sample(index).expect("valid").values()[1].to_bits()
        );
    }
    executor.shutdown().expect("shutdown");
}

#[test]
fn dispatch_surfaces_design_contract_violations() {
    let space = ParameterSpace::new([
        Parameter::borrowed("x", 0.0, 1.0).expect("valid"),
        Parameter::borrowed("y", 10.0, 20.0).expect("valid"),
    ])
    .expect("unique");
    let study = Study::borrowed("invalid-design", space, RejectingDesign).expect("named");
    let mut executor = HybridExecutor::new(ExecutorConfig {
        worker_threads: 2,
        ..ExecutorConfig::default()
    })
    .expect("executor");
    let mut output: Vec<Option<Result<f64, core::convert::Infallible>>> =
        (0..study.sample_count()).map(|_| None).collect();

    let error = MoiraiDispatch::<2>::new(&executor)
        .evaluate_into(&study, &BorrowingModel, &CopyResponse, &mut output)
        .expect_err("contract violation");
    assert!(matches!(
        error,
        DispatchError::DesignContract {
            index: 1,
            sample_count: 3
        }
    ));

    executor.shutdown().expect("shutdown");
}
