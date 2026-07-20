//! Symmetric scalar prediction interval.

/// A closed scalar prediction interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PredictionInterval<T> {
    lower: T,
    upper: T,
}

impl<T> PredictionInterval<T> {
    pub(crate) const fn new(lower: T, upper: T) -> Self {
        Self { lower, upper }
    }

    /// Lower endpoint.
    #[must_use]
    pub const fn lower(self) -> T
    where
        T: Copy,
    {
        self.lower
    }

    /// Upper endpoint.
    #[must_use]
    pub const fn upper(self) -> T
    where
        T: Copy,
    {
        self.upper
    }
}
