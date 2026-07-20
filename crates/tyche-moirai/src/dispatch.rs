//! Scoped, index-preserving study dispatch.

use eunomia::RealField;
use moirai_executor::{HybridExecutor, SyncTask};
use tyche_core::{Design, ResponseReducer, Study, StudyModel};

use crate::DispatchError;

/// A pointer-sized adapter borrowing the caller-owned Moirai executor.
///
/// `CHUNK` controls registration granularity at compile time. Trial outputs
/// remain in caller-owned, index-addressed slots and model failures stay
/// attached to their trial indices.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct MoiraiDispatch<'executor, const CHUNK: usize> {
    executor: &'executor HybridExecutor,
}

impl<'executor, const CHUNK: usize> MoiraiDispatch<'executor, CHUNK> {
    /// Borrow a Moirai executor without taking runtime lifecycle ownership.
    #[must_use]
    pub const fn new(executor: &'executor HybridExecutor) -> Self {
        Self { executor }
    }

    /// Evaluate every study sample into caller-owned result slots.
    ///
    /// Completed results are indexed independently of scheduler order.
    /// Aggregate them later in ascending slot order to preserve floating-point
    /// reproducibility across worker counts.
    ///
    /// # Errors
    ///
    /// Returns [`DispatchError`] for zero chunk width, mismatched output
    /// storage, or a Moirai scheduler failure. Model errors are values in their
    /// corresponding output slots.
    pub fn evaluate_into<'study, T, D, M, R, const PARAMETERS: usize>(
        &self,
        study: &Study<'study, T, D, PARAMETERS>,
        model: &M,
        reducer: &R,
        output: &mut [Option<Result<R::Output, M::Error>>],
    ) -> Result<(), DispatchError>
    where
        T: RealField,
        D: Design<PARAMETERS> + Sync,
        M: StudyModel<T, PARAMETERS>,
        R: ResponseReducer<M, T, PARAMETERS> + Sync,
        R::Output: Send,
        M::Error: Send,
    {
        if CHUNK == 0 {
            return Err(DispatchError::ZeroChunkWidth);
        }
        if output.len() != study.sample_count() {
            return Err(DispatchError::OutputLength {
                expected: study.sample_count(),
                actual: output.len(),
            });
        }

        self.executor.scope::<SyncTask, _>(|scope| {
            for (chunk_index, slots) in output.chunks_mut(CHUNK).enumerate() {
                let first_index = chunk_index * CHUNK;
                scope.spawn(move |_| {
                    for (offset, slot) in slots.iter_mut().enumerate() {
                        let sample = study
                            .sample(first_index + offset)
                            .expect("validated output domain matches study samples");
                        let result = model
                            .evaluate(sample.values())
                            .map(|response| reducer.reduce(response));
                        *slot = Some(result);
                    }
                })?;
            }
            Ok(())
        })?;
        Ok(())
    }
}
