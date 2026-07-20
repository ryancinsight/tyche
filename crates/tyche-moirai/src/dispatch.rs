//! Scoped, index-preserving study dispatch.

use crate::DispatchError;
use eunomia::RealField;
use moirai_executor::{HybridExecutor, SyncTask};
use tyche_core::{Design, ResponseReducer, Study, StudyModel};

/// Pointer-sized adapter borrowing a caller-owned Moirai executor.
#[must_use]
#[repr(transparent)]
pub struct MoiraiDispatch<'executor, const CHUNK: usize> {
    executor: &'executor HybridExecutor,
}

impl<'executor, const CHUNK: usize> MoiraiDispatch<'executor, CHUNK> {
    /// Borrow an executor without lifecycle ownership.
    pub const fn new(executor: &'executor HybridExecutor) -> Self {
        Self { executor }
    }

    /// Evaluate all samples into caller-owned indexed slots.
    ///
    /// # Errors
    ///
    /// Rejects zero chunk width, mismatched storage, or scheduler failure.
    /// Model errors remain values in their corresponding slots.
    ///
    /// # Panics
    ///
    /// Panics if a third-party [`Design`] implementation violates its contract
    /// by rejecting an index below its declared sample count.
    pub fn evaluate_into<T, D, M, R, const PARAMETERS: usize>(
        &self,
        study: &Study<'_, T, D, PARAMETERS>,
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
        output.fill_with(|| None);
        self.executor.scope::<SyncTask, _>(|scope| {
            for (chunk_index, slots) in output.chunks_mut(CHUNK).enumerate() {
                let first = chunk_index * CHUNK;
                scope.spawn(move |_| {
                    for (offset, slot) in slots.iter_mut().enumerate() {
                        let sample = study.sample(first + offset).expect(
                            "invariant: Design rejects only indices at or beyond sample_count",
                        );
                        *slot = Some(
                            model
                                .evaluate(sample.values())
                                .map(|response| reducer.reduce(response)),
                        );
                    }
                })?;
            }
            Ok(())
        })?;
        Ok(())
    }
}
