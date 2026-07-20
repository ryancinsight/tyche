//! Statically dispatched, borrow-preserving model contracts.

/// A model evaluated at fixed-width parameter samples.
///
/// The response GAT may borrow immutable configuration from the model and the
/// parameter sample. Execution adapters must reduce that response before the
/// borrow ends; it cannot escape into an untyped heap object.
pub trait StudyModel<T, const PARAMETERS: usize>: Sync {
    /// Model-specific evaluation failure.
    type Error;
    /// Borrowed or owned response family.
    type Response<'a>
    where
        Self: 'a,
        T: 'a;

    /// Evaluate one parameter sample.
    ///
    /// # Errors
    ///
    /// Returns the model's typed failure.
    fn evaluate<'a>(
        &'a self,
        parameters: &'a [T; PARAMETERS],
    ) -> Result<Self::Response<'a>, Self::Error>;
}

/// Convert a possibly borrowed model response into an owned result slot.
pub trait ResponseReducer<M, T, const PARAMETERS: usize>
where
    M: StudyModel<T, PARAMETERS>,
{
    /// Owned per-trial output.
    type Output;

    /// Reduce one response before its borrow ends.
    fn reduce<'a>(&self, response: M::Response<'a>) -> Self::Output
    where
        M: 'a,
        T: 'a;
}
