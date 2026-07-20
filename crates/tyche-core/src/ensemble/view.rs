//! Zero-copy ensemble response view.

use crate::statistics::{InsufficientSamples, Moments};
use eunomia::RealField;

/// Borrowed scalar responses in logical-index order.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Ensemble<'a, T> {
    responses: &'a [T],
}

impl<'a, T: RealField> Ensemble<'a, T> {
    /// Borrow a response slice.
    pub const fn new(responses: &'a [T]) -> Self {
        Self { responses }
    }

    /// Borrow responses.
    #[must_use]
    pub const fn responses(self) -> &'a [T] {
        self.responses
    }

    /// Summarize in logical order.
    ///
    /// # Errors
    ///
    /// Rejects an empty ensemble.
    pub fn moments(self) -> Result<Moments<T>, InsufficientSamples> {
        let mut moments = Moments::new();
        for &response in self.responses {
            moments.update(response);
        }
        if moments.is_empty() {
            Err(InsufficientSamples::new(1, 0))
        } else {
            Ok(moments)
        }
    }
}
