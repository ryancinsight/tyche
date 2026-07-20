//! Experimental-design abstraction over a unit hypercube.

use core::fmt;

/// A deterministic random-access design over a unit hypercube.
pub trait Design<const PARAMETERS: usize> {
    /// Number of sample points in the design.
    fn sample_count(&self) -> usize;

    /// Fill `output` with the normalized point at `index`.
    ///
    /// # Errors
    ///
    /// Returns [`SampleIndexError`] when `index >= self.sample_count()`.
    fn sample_unit_into(
        &self,
        index: usize,
        output: &mut [f64; PARAMETERS],
    ) -> Result<(), SampleIndexError>;
}

/// A requested sample index lies outside its design.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleIndexError {
    index: usize,
    sample_count: usize,
}

impl SampleIndexError {
    pub(crate) const fn new(index: usize, sample_count: usize) -> Self {
        Self {
            index,
            sample_count,
        }
    }

    /// Invalid index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Design sample count.
    #[must_use]
    pub const fn sample_count(self) -> usize {
        self.sample_count
    }
}

impl fmt::Display for SampleIndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "sample index {} is outside design length {}",
            self.index, self.sample_count
        )
    }
}

impl core::error::Error for SampleIndexError {}
