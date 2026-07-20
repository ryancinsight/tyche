//! Statically dispatched, borrow-preserving model contracts.

/// A model evaluated at fixed-width parameter samples.
pub trait StudyModel<T, const PARAMETERS: usize>: Sync {
    /// Model failure.
    type Error;
    /// Borrowed or owned response family.
    type Response<'a>
    where
        Self: 'a,
        T: 'a;

    /// Evaluate one sample.
    ///
    /// # Errors
    ///
    /// Returns the model's typed failure.
    fn evaluate<'a>(
        &'a self,
        parameters: &'a [T; PARAMETERS],
    ) -> Result<Self::Response<'a>, Self::Error>;
}

/// Convert a possibly borrowed response into an owned result slot.
pub trait ResponseReducer<M, T, const PARAMETERS: usize>
where
    M: StudyModel<T, PARAMETERS>,
{
    /// Owned output.
    type Output;

    /// Reduce before the response borrow ends.
    fn reduce<'a>(&self, response: M::Response<'a>) -> Self::Output
    where
        M: 'a,
        T: 'a;
}
