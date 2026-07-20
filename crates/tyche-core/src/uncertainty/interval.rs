//! Symmetric scalar prediction interval.

/// Closed scalar interval.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PredictionInterval<T> {
    lower: T,
    upper: T,
}

impl<T: Copy> PredictionInterval<T> {
    pub(crate) const fn new(lower: T, upper: T) -> Self {
        Self { lower, upper }
    }
    /// Lower endpoint.
    #[must_use]
    pub const fn lower(self) -> T {
        self.lower
    }
    /// Upper endpoint.
    #[must_use]
    pub const fn upper(self) -> T {
        self.upper
    }
}
