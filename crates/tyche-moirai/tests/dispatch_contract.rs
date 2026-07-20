//! Moirai execution adapter contracts.

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

fn study() -> Study<'static, f64, LatinHypercube<2>, 2> {
    let space = ParameterSpace::new([
        Parameter::borrowed("x", 0.0, 1.0).expect("valid"),
        Parameter::borrowed("y", 10.0, 20.0).expect("valid"),
    ])
    .expect("unique");
    let design = LatinHypercube::new(
        Seed::new(44),
        NonZeroUsize::new(257).expect("constant is positive"),
    );
    Study::borrowed("parallel replay", space, design).expect("named")
}

#[test]
fn scoped_moirai_dispatch_preserves_indexed_results() {
    let mut executor = HybridExecutor::new(ExecutorConfig {
        worker_threads: 2,
        ..ExecutorConfig::default()
    })
    .expect("executor");
    let study = study();
    let mut output: Vec<Option<Result<f64, core::convert::Infallible>>> =
        (0..study.sample_count()).map(|_| None).collect();

    MoiraiDispatch::<7>::new(&executor)
        .evaluate_into(&study, &BorrowingModel, &CopyResponse, &mut output)
        .expect("dispatch");

    for (index, slot) in output.into_iter().enumerate() {
        let actual = slot
            .expect("every slot initialized")
            .expect("infallible model");
        let expected = study.sample(index).expect("valid sample").values()[1];
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    executor.shutdown().expect("shutdown");
}

#[test]
fn mismatched_output_is_rejected_before_dispatch() {
    let executor = HybridExecutor::new(ExecutorConfig {
        worker_threads: 1,
        ..ExecutorConfig::default()
    })
    .expect("executor");
    let study = study();
    let mut output: [Option<Result<f64, core::convert::Infallible>>; 1] = [None];

    let error = MoiraiDispatch::<4>::new(&executor)
        .evaluate_into(&study, &BorrowingModel, &CopyResponse, &mut output)
        .expect_err("length mismatch");

    assert!(matches!(
        error,
        tyche_moirai::DispatchError::OutputLength {
            expected: 257,
            actual: 1
        }
    ));
}
