//! Shared integration-test fixtures for the Tyche test crates.

use tyche_core::{ResponseReducer, StudyModel};

/// A model that evaluates to a borrow of one parameter slot.
///
/// `INDEX` selects the slot; the cross-crate contract tests instantiate it
/// with `0` and `1` to prove the reducer copies rather than borrows.
pub struct BorrowingModel<const INDEX: usize>;

impl<const INDEX: usize> StudyModel<f64, 2> for BorrowingModel<INDEX> {
    type Error = core::convert::Infallible;
    type Response<'a> = &'a f64;

    /// Evaluate one sample.
    ///
    /// # Panics
    ///
    /// Panics if `INDEX` is at least the parameter count `2`.
    fn evaluate<'a>(&'a self, parameters: &'a [f64; 2]) -> Result<Self::Response<'a>, Self::Error> {
        Ok(&parameters[INDEX])
    }
}

/// A reducer that copies a borrowed response into an owned `f64`.
pub struct CopyResponse;

impl<const INDEX: usize> ResponseReducer<BorrowingModel<INDEX>, f64, 2> for CopyResponse {
    type Output = f64;

    fn reduce<'a>(
        &self,
        response: <BorrowingModel<INDEX> as StudyModel<f64, 2>>::Response<'a>,
    ) -> Self::Output
    where
        BorrowingModel<INDEX>: 'a,
        f64: 'a,
    {
        *response
    }
}
