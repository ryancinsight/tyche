//! Moirai adapter contracts.

use core::num::NonZeroUsize;
use moirai_core::executor::ExecutorConfig;
use moirai_executor::HybridExecutor;
use tyche_core::{
    LatinHypercube, Parameter, ParameterSpace, ResponseReducer, Seed, Study, StudyModel,
};
use tyche_moirai::MoiraiDispatch;

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
    fn reduce<'a>(&self, response: &'a f64) -> Self::Output {
        *response
    }
}

#[test]
fn dispatch_preserves_logical_indices() {
    let space = ParameterSpace::new([
        Parameter::borrowed("x", 0.0, 1.0).expect("valid"),
        Parameter::borrowed("y", 10.0, 20.0).expect("valid"),
    ])
    .expect("unique");
    let design = LatinHypercube::new(Seed::new(44), NonZeroUsize::new(257).expect("positive"));
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
